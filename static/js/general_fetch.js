function general_fetch(url, data, method="GET", body=undefined) {
    fetch(url, {
        method: method,
        headers: {
            "Content-Type": "application/json"
        },
        body: body
    })
        .then(response => {
            if (response.status === 204) {
                return response.text();
            }

            if (response.ok) {
                return response.json();
            }

            document.getElementById("error").show();
            throw new Error(`HTTP error! status: ${response.status}`);
        })
        .then(data)
        .catch(error => {
            if (error !== undefined) {
                console.error(error);
            }
        });
}