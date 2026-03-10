use actix_web::{
    App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
    body::BoxBody,
    http::{StatusCode, header},
    web,
};
use image::{ImageBuffer, Rgb};
use qrcode::{EcLevel, QrCode};
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
    let mut format = OutputFormat::Png;
    let mut send_data_format = SendDataFormat::BrowserDisplay;
    let mut ecl = None;
    if req
        .headers()
        .get(header::USER_AGENT)
        .map(|s| s.to_str().ok())
        .flatten()
        .map(|s| s.contains("curl"))
        .unwrap_or(false)
    {
        format = OutputFormat::Text;
    }
    let mut target = "https://github.com/Johannes-Pabst/qr_code_api/".to_string();
    if let Some(fsidm1) = url.find("/") {
        let fsid = fsidm1 + 1;
        let url = url[fsid..].to_string();
        let ssid = url.find("/").unwrap_or(url.len());
        let farg = url[0..ssid].to_string();
        if farg
            .chars()
            .all(|c| "abcdefghijklmnopqrstuvwxyz-".contains(c))
            && ssid < url.len()
        {
            target = url[ssid + 1..].to_string();
            let flags = farg.split('-').collect::<Vec<&str>>();
            for flag in flags {
                match flag {
                    "png" => format = OutputFormat::Png,
                    "jpg" => format = OutputFormat::Jpg,
                    "svg" => format = OutputFormat::Svg,
                    "text" => format = OutputFormat::Text,
                    "low" => ecl = Some(EcLevel::L),
                    "medium" => ecl = Some(EcLevel::M),
                    "quartile" => ecl = Some(EcLevel::Q),
                    "high" => ecl = Some(EcLevel::H),
                    "download" => send_data_format = SendDataFormat::Download,
                    "browser" => send_data_format = SendDataFormat::BrowserDisplay,
                    _ => {
                        return HttpResponse::BadRequest().body("Invalid format or flag provided.");
                    }
                };
            }
        } else {
            target = url;
        }
    }
    let code_err = if let Some(ecl) = ecl {
        QrCode::with_error_correction_level(&target, ecl)
    } else {
        QrCode::new(&target)
    };
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
                target.replace("&", "&amp;"),
                image[id1..id2].to_string(),
                image[id2..].to_string()
            );
            (BoxBody::new(image), "image/svg+xml", "svg")
        }
        OutputFormat::Text => {
            let image = code
                .render()
                .light_color('w')
                .dark_color('b')
                .quiet_zone(false)
                .build();
            let repeat = "b".repeat(image.find('\n').unwrap());
            let lines = image
                .lines()
                .chain(std::iter::once(repeat.as_str()))
                .collect::<Vec<&str>>();
            let comb = lines
                .chunks(2)
                .map(|c| {
                    format!(
                        "{}\n",
                        c[0].chars()
                            .zip(c[1].chars())
                            .map(|(c1, c2)| match (c1, c2) {
                                ('b', 'b') => ' ',
                                ('b', 'w') => '▄',
                                ('w', 'b') => '▀',
                                ('w', 'w') => '█',
                                (_, _) => panic!(),
                            })
                            .collect::<String>()
                    )
                })
                .collect::<String>();

            (BoxBody::new(comb), "text/plain; charset=utf8", "txt")
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
