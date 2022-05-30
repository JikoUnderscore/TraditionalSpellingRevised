def gen():
    TSR_dict: dict[str, str] = {}
    with open("./dictionary/TSR.dict", 'r') as f:
        word: str
        for word in f:
            kv: list[str] = word.strip().split(' ')

            assert len(kv) == 2
            TSR_dict[kv[0]] = kv[1]

    with open("./dictionary/UNCHANGED.dict", 'r') as f:
        word: str
        for word in f:
            kv: list[str] = word.strip().split(' ')

            assert len(kv) == 2
            TSR_dict[kv[0]] = kv[1]


    with open("./js/dict.js", "w") as js:
        js.write("export const DICT = new Map([\n")

        with open("./ESR/src/static_map.rs", "w") as rs:
            rs.write("pub static DICT: phf::Map<&'static str, &'static str> = phf::phf_map! {\n")

            for key, value in TSR_dict.items():
                rs.write(f'\t"{key}" => "{value}",\n')
                js.write(f'\t["{key}", "{value}"],\n')

            rs.write(f'\t" " => " ",\n')
            rs.write(f'\t"i" => "I",\n')
            rs.write("};")
        js.write('\t["i", "I"],\n')
        js.write("]);")


if __name__ == '__main__':
    gen()

