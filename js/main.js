import {DICT} from "./dict.js";

const text_box_input = document.getElementById("input");

text_box_input.addEventListener('input', input, false);

let div = document.getElementById('out_put');

function input() {
    div.innerHTML = translation(text_box_input.value);
}


function translation(text) {
    const punct = "'!@#█½-$%’'^&*( '){}[]¿|._-\`/?:;«»‹›—\,“”~";

    for (const pun of punct) {
        text = text.replace(pun, " " + pun);
    }


    let ntext = text.toLowerCase().split(" ");

    let new_text = "";
    for (let word of ntext) {
        console.log(word.raw());

        let got = DICT.get(word)
        if (got === undefined) {
            if (word !== "") {

                new_text += " <span style=\"color: #fa0000;\">" + word + "</span>"; // red are unconfurmed words
            }
        } else {
            if (got === word) {
                new_text += " " + got; // black comfured words with no change of spelling
            } else {
                new_text += " <span style=\"color: #5a65a8;\">" + got + "</span>"; // blue comfurmed word with changed spelling
            }
        }
    }


    return new_text;
}


