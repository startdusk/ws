mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlButtonElement;

use models::course::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    fn confirm(s: &str) -> bool;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-client!");
}

pub mod errors;
pub mod models;

use models::course::Course;

#[wasm_bindgen(start)] // start 表示项目一启动就会执行这个函数
pub async fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no global document exists");

    let left_tbody = document
        .get_element_by_id("left-tbody")
        .expect("left_div_not_exists");

    let courses: Vec<Course> = models::course::get_courses_by_teacher(1).await.unwrap();
    for c in courses.iter() {
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;
        let id = document.create_element("id")?;
        id.set_text_content(Some(format!("{}", c.id).as_str()));
        tr.append_child(&id)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.name.as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(desc) = c.description.clone() {
            td.set_text_content(Some(desc.as_str()));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        let btn: HtmlButtonElement = document
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let cid = c.id;
        let click_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let r = confirm(format!("确认删除 ID 为{} 的课程?", cid).as_str());
            match r {
                true => {
                    spawn_local(delete_course(1, cid));
                    alert("删除成功!");

                    web_sys::window().unwrap().location().reload().unwrap();
                }
                _ => {}
            }
        }) as Box<dyn Fn(_)>);
        btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget(); // 走出作用域后，闭包仍然有效
        btn.set_attribute("type", "button")?;
        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("Delete"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_tbody.append_child(&tr)?;
    }

    Ok(())
}
