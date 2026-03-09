use actix_web::{
    App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder, body::BoxBody,
    http::StatusCode, web,
};
use image::{ImageBuffer, Rgb};
use qrcode::QrCode;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = HttpServer::new(|| App::new().default_service(web::to(generate)))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await;
    Ok(())
}
async fn generate(req: HttpRequest) -> impl Responder {
    let url = req.full_url().to_string();
    let url = if url.starts_with("http://") {
        url.replacen("http://", "", 1)
    } else if url.starts_with("https://") {
        url.replacen("https://", "", 1)
    } else {
        url
    };
    let fsid = url.find("/").unwrap() + 1;
    let url = url[fsid..].to_string();
    let ssid = url.find("/").unwrap_or(url.len());
    let farg = url[0..ssid].to_string();
    let url = url[ssid + 1..].to_string();
    let flags = farg.split('-').collect::<Vec<&str>>();
    let mut format = OutputFormat::Png;
    let mut send_data_format = SendDataFormat::BrowserDisplay;
    for flag in flags {
        match flag {
            "png" => format = OutputFormat::Png,
            "jpg" => format = OutputFormat::Jpg,
            "svg" => format = OutputFormat::Svg,
            "text" => format = OutputFormat::Text,
            "download" => send_data_format = SendDataFormat::Download,
            "browser" => send_data_format = SendDataFormat::BrowserDisplay,
            _ => return HttpResponse::BadRequest().body("Invalid format or flag provided."),
        };
    }
    let code_err = QrCode::new(&url);
    if code_err.is_err() {
        return HttpResponseBuilder::new(StatusCode::PAYLOAD_TOO_LARGE)
            .body("Failed to generate QR code, probably too long URL or invalid characters.");
    }
    let code = code_err.unwrap();
    let (data, content_type, extension) = match format {
        OutputFormat::Png => {
            let image = code.render::<Rgb<u8>>().quiet_zone(false).build();
            let (w, h) = image.dimensions();
            let buffer: ImageBuffer<Rgb<u8>, _> =
                ImageBuffer::from_raw(w, h, image.into_raw().to_vec()).unwrap();
            let mut data = Vec::new();
            let encoder = image::codecs::png::PngEncoder::new(&mut data);
            buffer.write_with_encoder(encoder).unwrap();
            (BoxBody::new(data), "image/png", "png")
        }
        OutputFormat::Jpg => {
            let image = code.render::<Rgb<u8>>().quiet_zone(false).build();
            let (w, h) = image.dimensions();
            let buffer: ImageBuffer<Rgb<u8>, _> =
                ImageBuffer::from_raw(w, h, image.into_raw().to_vec()).unwrap();
            let mut data = Vec::new();
            let encoder = image::codecs::jpeg::JpegEncoder::new(&mut data);
            buffer.write_with_encoder(encoder).unwrap();
            (BoxBody::new(data), "image/jpg", "jpg")
        }
        OutputFormat::Svg => {
            let image = code
                .render::<qrcode::render::svg::Color>()
                .quiet_zone(false)
                .build();
            let id1 = image.find("\">").unwrap() + 2;
            let id2 = image.find("</svg>").unwrap();
            let image = format!(
                "{}<a href=\"{}\" target=\"_blank\">{}</a>{}",
                image[..id1].to_string(),
                url.replace("&", "&amp;"),
                image[id1..id2].to_string(),
                image[id2..].to_string()
            );
            (BoxBody::new(image), "image/svg+xml", "svg")
        }
        OutputFormat::Text => {
            let image = code
                .render().light_color('w').dark_color('b')
                .quiet_zone(false)
                .build();
            let repeat = "b".repeat(image.find('\n').unwrap());
            let lines=image.lines().chain(std::iter::once(repeat.as_str())).collect::<Vec<&str>>();
            let comb=lines.chunks(2).map(|c| {
                c[0].chars().zip(c[1].chars()).map(|(c1, c2)| match (c1, c2) {
                    ('b', 'b')=>' ',
                    ('b', 'w')=>'▄',
                    ('w', 'b')=>'▀',
                    ('w', 'w')=>'█',
                    (_, _) => panic!()
                }).collect::<String>()
            }).collect::<Vec<_>>().join("\n");
            
            (BoxBody::new(comb), "text/plain", "txt")
        }
    };
    HttpResponse::Ok()
        .content_type(content_type)
        .append_header((
            "Content-Disposition",
            match send_data_format {
                SendDataFormat::BrowserDisplay => "inline".to_string(),
                SendDataFormat::Download => {
                    format!("attachment; filename=\"qrcode.{}\"", extension)
                }
            },
        ))
        .body(data)
}
enum SendDataFormat {
    BrowserDisplay,
    Download,
}
enum OutputFormat {
    Png,
    Jpg,
    Svg,
    Text,
}
/*
    url: {url}

    contact info:
    BEGIN:VCARD
VERSION:3.0
N:b;a
FN:a b
ORG:c
TITLE:d
ADR:;;e;f;g;4;h
TEL;WORK;VOICE:2
TEL;CELL:1
TEL;FAX:3
EMAIL;WORK;INTERNET:a@a.com
URL:e.com
END:VCARD

    send email: MATMSG:TO:{address};SUB:{subject};BODY:{message};;

    send sms: SMSTO:{number}:{text}

    enter wifi: WIFI:T:{WPA};S:{ssid};P:{password};H:{true};;

    transfer crypto currency: {currency}:{address}?amount={amount}&message={message}
*/
