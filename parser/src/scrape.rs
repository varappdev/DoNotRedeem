use std::{collections::HashMap, fmt::Error};

use dom_query::Document;
use dom_smoothie::{Article, Config, Readability};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Security {
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interactives {
    pub links: Vec<String>,
    pub buttons: Vec<String>,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerialisedDocument {
    pub security: Security,
    pub content: String,
    pub interactives: Interactives,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleSerialisedDocument {
    pub content: String,
    pub security: Security,
}

// Currently
pub fn serialise(html: &str, url: &str) -> Result<SimpleSerialisedDocument, Error> {
    // let doc = Document::from(html);

    // Add specific parsing here

    let mut readability = Readability::new(html, Some(url), None).map_err(|_| Error)?;
    let article = readability.parse().map_err(|_| Error)?;
    let content = htmd::convert(&article.content).map_err(|_| Error)?;

    Ok(SimpleSerialisedDocument {
        content: content,
        security: Security {
            domain: url.to_string(),
        },
    })
}

pub fn serialise_content(html: &str) -> Result<String, Error> {
    let readability = Readability::new(html, None, None);
    let article = readability.unwrap().parse();

    Ok(htmd::convert(&article.unwrap().content).unwrap())
}

impl From<SimpleSerialisedDocument> for HashMap<String, String> {
    fn from(value: SimpleSerialisedDocument) -> Self {
        let mut map = HashMap::new();
        map.insert(
            "security".to_owned(),
            serde_json::to_string_pretty(&value.security).unwrap(),
        );
        map.insert("content".to_owned(), value.content);

        map
    }
}

// Returns a string array of links from various sources
// fn get_links(doc: &Document) -> Vec<String> {
//     let a = doc.
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_md() {
        let result = serialise_content(
            r#"
            <!doctype html>
            <html lang="en-GB">
            <head>
              <meta charset="utf-8">
              <meta name="viewport" content="width=device-width, initial-scale=1">
              <title>100 FREE SPINS – ANDROMEDASINO UK</title>
            </head>

            <body style="margin:0;padding:20px 10px;background:#090514;font-family:'Segoe UI', Arial, sans-serif;color:#ffffff;">

            <center>

            <!-- MAIN CONTAINER -->
            <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0"
              style="width:100%;max-width:520px;background:linear-gradient(180deg, #160c2e 0%, #080312 100%);border-radius:24px;overflow:hidden;border:2px solid #a855f7;box-shadow:0 0 40px rgba(168,85,247,0.35);">

              <!-- URGENCY HEADER -->
              <tr>
                <td align="center" style="background:linear-gradient(90deg, #7e22ce 0%, #a855f7 50%, #7e22ce 100%);padding:12px;font-size:12px;font-weight:900;color:#ffffff;letter-spacing:1.5px;text-transform:uppercase;">
                  🔥 UK EXCLUSIVE OFFER 🇬🇧 — NO DEPOSIT REQUIRED
                </td>
              </tr>

              <!-- LOGO -->
              <tr>
                <td align="center" style="padding:28px 20px 10px;">
                  <div style="font-size:32px;font-weight:900;letter-spacing:1.5px;color:#ffffff;text-transform:uppercase;line-height:1;">
                    ANDROMEDASINO <span style="color:#f59e0b;">CASINO</span>
                  </div>
                </td>
              </tr>

              <!-- HERO OFFER CARD -->
              <tr>
                <td style="padding:15px 20px;">
                  <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0"
                    style="background:#0f0720;border-radius:20px;border:2px dashed #f59e0b;padding:25px 15px;">
                    <tr>
                      <td align="center">

                        <div style="display:inline-block;background:#f59e0b;color:#0f0720;padding:6px 18px;border-radius:50px;font-size:11px;font-weight:900;letter-spacing:1px;text-transform:uppercase;margin-bottom:12px;">
                          🎁 100% FREE BONUS
                        </div>

                        <div style="font-size:68px;font-weight:900;line-height:0.9;color:#ffffff;letter-spacing:-2px;margin:8px 0;text-shadow:0 0 25px rgba(255,255,255,0.3);">
                          100 <span style="color:#a855f7;">SPINS</span>
                        </div>

                        <div style="font-size:26px;font-weight:900;color:#f59e0b;letter-spacing:2px;text-transform:uppercase;margin-bottom:12px;">
                          FREE ROUNDS NOW 🎰
                        </div>

                        <div style="font-size:14px;color:#cbd5e1;line-height:1.5;max-width:380px;font-weight:500;">
                          Create your account in 30 seconds at <strong style="color:#ffffff;">Andromedasino</strong> and claim your <strong style="color:#f59e0b;">100 Free Bonus Spins</strong>. Spin top UK slots with zero risk!
                        </div>

                      </td>
                    </tr>
                  </table>
                </td>
              </tr>

              <!-- BIG CTA BUTTON -->
              <tr>
                <td align="center" style="padding:10px 20px 20px;">
                  <table role="presentation" width="100%" cellspacing="0" cellpadding="0" border="0" style="max-width:400px;">
                    <tr>
                      <td align="center" style="background:linear-gradient(135deg, #f59e0b 0%, #d97706 100%);border-radius:16px;box-shadow:0 0 30px rgba(245,158,11,0.5);">
                        <a href="https://cutt.ly/LyQ0XK1F#4voLhI24788DKqt159kyvqordofw16TGUJHVYQDUXGORR129417ZHEI2489t16" style="display:block;padding:20px 15px;color:#0f0720;text-decoration:none;font-size:20px;line-height:1.2;font-weight:900;letter-spacing:1px;text-transform:uppercase;text-align:center;">
                          🔥 CLAIM 100 FREE SPINS →
                        </a>
                      </td>
                    </tr>
                  </table>
                  <div style="font-size:12px;color:#94a3b8;margin-top:10px;font-weight:700;">
                    ⚡️ Instant Setup • No Credit Card Required
                  </div>
                </td>
              </tr>

              <!-- PROOF TICKER -->
              <tr>
                <td align="center" style="padding:0 20px 20px;">
                  <div style="background:#13092a;border:1px solid #3b0764;border-radius:12px;padding:10px 15px;font-size:12px;color:#cbd5e1;">
                    💬 <strong style="color:#a855f7;">Oliver P. (London):</strong> "Signed up, got 100 free spins immediately and won £340 without spending a penny!"
                  </div>
                </td>
              </tr>

              <!-- SECONDARY LINK -->
              <tr>
                <td align="center" style="padding:0 20px 25px;">
                  <a href="https://cutt.ly/LyQ0XK1F#4ytGzP24788poaP159kxhmymczmx16ATVBBGFZDROXMPP129417LIRL2489U16" style="font-size:13px;font-weight:900;color:#f59e0b;text-decoration:none;letter-spacing:1px;text-transform:uppercase;border-bottom:1px solid #f59e0b;padding-bottom:2px;">
                    👉 PLAY NOW AT ANDROMEDASINO &raquo;
                  </a>
                </td>
              </tr>

              <!-- FOOTER -->
              <tr>
                <td align="center" style="padding:20px;background:#040208;font-size:10px;color:#64748b;line-height:1.5;">
                  BeGambleAware.org • Strictly 18+ UK players only. Play responsibly.<br>
                  <a href="https://cutt.ly/LyQ0XK1F#4FyJGX24788zRWP159onaefhulaj16VYPDXYGLBXLTIHX129417HFDN2489C16" style="color:#94a3b8;text-decoration:underline;">Unsubscribe</a>
                  &nbsp;•&nbsp;
                  <a href="https://cutt.ly/LyQ0XK1F#4yVUBH24788ayzI159rwehgwyelv16FEMNDBGTTWAXTCD129417DUVK2489m16" style="color:#94a3b8;text-decoration:underline;">Terms & Conditions</a>
                </td>
              </tr>

            </table>

            </center>
            </body>
            </html>
            "#,
        );

        dbg!(&result);
    }
}
