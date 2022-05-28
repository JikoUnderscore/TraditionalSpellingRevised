def gen():
    with open("./dictionary/TSR.dict", 'r') as f:

        with open("./ESR/src/static_map.rs", "w") as fp:
            fp.write("pub static DICT: phf::Map<&'static str, &'static str> = phf::phf_map! {\n")
            for word in f:
                kv = word.strip().split(' ')
                assert len(kv) == 2
                # print(kv)
                fp.write(f'\t"{kv[0]}" => "{kv[1]}",\n')
            fp.write(f'\t" " => " ",\n')
            fp.write(f'\t"i" => "I",\n')
            fp.write("};")


if __name__ == '__main__':
    gen()

