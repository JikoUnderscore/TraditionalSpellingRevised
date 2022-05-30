import {DICT} from "./dict.js";

const text_box_input = document.getElementById("input");

text_box_input.addEventListener('input', input, false);

let div = document.getElementById('TEST');

function input() {
    div.innerHTML = translation(text_box_input.value);
}


function translation(text) {
    let ntext = text.toLowerCase().split(" ")

    let new_text = "";
    for (let i = 0; i < ntext.length; i++) {
        let got = DICT.get(ntext[i])
        if (got === undefined) {
            if (ntext[i] !== "") {
                new_text += " <span style=\"color: #e83939;\">" + ntext[i] + "</span>"; // red are unconfurmed words
            }
        } else {
            if (got === ntext[i]) {
                new_text += " " + got; // black comfured words with no change of spelling
            } else {
                new_text += " <span style=\"color: #5a65a8;\">" + got + "</span>"; // blue comfurmed word with changed spelling
            }
        }
    }


    return new_text;
}


