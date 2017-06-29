#[derive(Debug, Clone, Default, Serialize)]
pub struct Message {
    pub username: Option<String>,
    pub icon_emoji: Option<String>,
    pub text: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            ..Default::default()
        }
    }

    pub fn with_username(mut self, username: String) -> Message {
        self.username = Some(username);
        self
    }

    pub fn with_icon_emoji(mut self, icon_emoji: String) -> Message {
        self.icon_emoji = Some(icon_emoji);
        self
    }

    pub fn with_text(mut self, text: String) -> Message {
        self.text = Some(text);
        self
    }

    pub fn with_attachments(mut self, attachments: Vec<Attachment>) -> Message {
        self.attachments = Some(attachments);
        self
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        match self.attachments {
            Some(ref mut attachments) => {
                attachments.push(attachment);
            },
            None => {
                self.attachments = Some(vec!(attachment));
            },
        };
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    pub title: String,
    pub title_link: String,
    pub text: String,
    pub image_url: String,
    pub color: String,
}
