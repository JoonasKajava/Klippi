import "../styles.css";
import "@fortawesome/fontawesome-free/css/brands.css";
import "@fortawesome/fontawesome-free/css/solid.css";
import "@fortawesome/fontawesome-free/css/regular.css";
import "@fortawesome/fontawesome-free/css/fontawesome.css";
export const prerender = true
export const ssr = false

export function load({url}) {
    return {
        url: url.pathname
    };
}