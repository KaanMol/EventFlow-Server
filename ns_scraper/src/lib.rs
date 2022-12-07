use headless_chrome::Browser;

pub async fn scrape(url: String) -> String {
    let browser = Browser::default().unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to(url.as_str()).unwrap();

    let elem = tab
        .wait_for_element(".rio-jp-travel-option-active .rio-jp-travel-time > time")
        .unwrap()
        .get_inner_text()
        .unwrap();

    elem
}
