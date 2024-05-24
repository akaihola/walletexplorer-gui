// These globals will be injected into a page that will use them.
/* eslint no-unused-vars: "off" */

// This is quite old and I don't want to waste too much time here. We probably
// should stop using this altogether as the examples should be easy and
// straightforward to understand and this only obscures it.
/* eslint require-jsdoc: "off" */

/* global Alea:false seededRandom:true */

/**
 * Created by Alex on 5/20/2015.
 *
 * @remarks
 * This depends on Alea from https://unpkg.com/alea@1.0.0/alea.js.
 */

/**
 * @param path
 * @param success
 * @param error
 */
function loadJSON(path, success, error) {
    const xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4) {
            if (xhr.status === 200) {
                success(JSON.parse(xhr.responseText));
            } else {
                error(xhr);
            }
        }
    };
    xhr.open("GET", path, true);
    xhr.send();
}
