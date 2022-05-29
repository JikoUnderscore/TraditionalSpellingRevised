import { DICT } from "./dict.js";

const text_box_input = document.getElementById("input");
let text_output = document.getElementById("output");

text_box_input.addEventListener('input', input, false);

function input() {
    console.log("thiss is a cat");
    text_output.value = translation(text_box_input.value);

}


function translation(text) {
    let ntext = text.split(" ")

    let new_text = "";
    for (let i = 0; i < ntext.length; i++) {
        console.log("ntext[i]", ntext[i]);

        let got = DICT.get(ntext[i])
        if (got === undefined){
            if (ntext[i] !== ""){
                new_text += " #" + ntext[i] + "#";
            }
        } else {
            new_text += " " + got;
        }
    }


    return new_text;
}


