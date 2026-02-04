function setCookie(name, value, max_age=0, secure=false, path="/", expires="", domain="", samesite="Lax") {
    let cookie = name + "=" + value + "; ";

    cookie += expires === "" ? "" : `expires=${expires}; `;
    cookie += max_age <= 0 ? "" : `max-age=${max_age};`;
    cookie += domain === "" ? "" : `domain=${domain}; `;
    cookie += `path=${path}; `;
    cookie += secure ? "Secure; " : "";
    cookie += `samesite=${samesite}; `;

    document.cookie = cookie;
}

function getCookie(name) {
    let cookies = document.cookie.split(";");

    for (let cookie of cookies) {
        cookie = cookie.trim();

        let equal = cookie.indexOf("=");

        if (cookie.substring(0, equal) === name) {
            return cookie.substring(equal +1);
        }
    }

    return "";
}

function deleteCookie(name) {
    document.cookie = `${name}=; max-age=0`;
}
