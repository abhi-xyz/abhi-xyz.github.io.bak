use crate::home_page::{Footer, Header};
use yew::prelude::*;

#[function_component(ArtGallary)]
pub fn art_gallary() -> Html {
    html! {
    <>
    <Header />
            <h1 id="articleH1">{ "Art Gallary" }</h1>
    <div class="art_gallary_wrapper">
    <div class="art_gallary_list">
        <img src="../../../assets/images/art_gallary/jhonnydepp.avif" alt="" />
        <img src="../../../assets/images/art_gallary/girl02.avif" alt="" />
        <img src="../../../assets/images/art_gallary/ronaldo.avif" alt="" />
        <img src="../../../assets/images/art_gallary/jai.avif" alt="" />
        <img src="../../../assets/images/art_gallary/rdj.avif" alt="" />
        <img src="../../../assets/images/art_gallary/elephant.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/subin.avif" alt="" />
        <img src="../../../assets/images/art_gallary/vidy.avif" alt="" />
        <img src="../../../assets/images/art_gallary/girl03.avif" alt="" />
        <img src="../../../assets/images/art_gallary/hrithik.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/girl01.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/jacksparrow.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/life_is_strange.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/messi.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/hrithik02.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/tiger02.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/anti_raging.avif" alt="" />
        <img src="../../../assets/images/art_gallary/shraddha.avif" alt="" />
        <img src="../../../assets/images/art_gallary/depika.avif" alt="" />
    //   <img src="../../../assets/images/art_gallary/sonic.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/tiger.avif" alt="" />
    //    <img src="../../../assets/images/art_gallary/tatoo.avif" alt="" />
    </div>
    </div>

    <Footer />
    </>
    }
}
