use dioxus::prelude::*;
use dxc::prelude::*;

#[component]
pub fn LayoutAhmf() -> Element {
    rsx! {
      div {
          class: "common-layout",
          DxcContainer {
          class: "container",
          DxcAside {
            width: "200px"
           }
           DxcContainer {
            direction: container::Direction::Vertical,
            DxcHeader {

             }
             DxcMain{
             }
             DxcFooter {
              }
            }
         }
         }
    }
}
