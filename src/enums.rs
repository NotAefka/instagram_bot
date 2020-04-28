#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Selector {
    Css,
    XPath,
    TagName,
    LinkText,
    PartialLinkText,
}

impl From<Selector> for &'static str {
    fn from(selector: Selector) -> &'static str {
        match selector {
            Selector::Css => "css selector",
            Selector::XPath => "xpath",
            Selector::TagName => "tag name",
            Selector::LinkText => "link text",
            Selector::PartialLinkText => "partial link text",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Browser {
    Firefox,
    Chrome,
}

impl From<Browser> for &'static str {
    fn from(browser: Browser) -> &'static str {
        match browser {
            Browser::Firefox => "firefox",
            Browser::Chrome => "chrome",
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Platform {
    Linux,
    Windows,
    Unknow,
}

impl Platform {
    pub fn current() -> Platform {
        if cfg!(unix) {
            Platform::Linux
        } else if cfg!(windows) {
            Platform::Windows
        } else {
            Platform::Unknow
        }
    }
}

impl From<Platform> for &'static str {
    fn from(platform: Platform) -> &'static str {
        match platform {
            Platform::Linux => "linux",
            Platform::Windows => "windows",
            Platform::Unknow => "unknow",
        }
    }
}

pub trait WebdriverObject: PartialEq {
    fn get_id(&self) -> &String;
}
