import gen_static_map as gen

def add_new_word():
    TSR_dict: dict[str, str] = {}
    with open("./dictionary/TSR.dict", 'r', encoding="utf8") as f:
        word: str
        for word in f:
            kv: list[str] = word.strip().split(' ')

            assert len(kv) == 2
            TSR_dict[kv[0]] = kv[1]

    with open("./dictionary/UNCHANGED.dict", 'r', encoding="utf8") as f:
        word: str
        for word in f:
            kv: list[str] = word.strip().split(' ')

            assert len(kv) == 2
            TSR_dict[kv[0]] = kv[1]

    with open("./dictionary/new_words", 'r', encoding="utf8") as f:
        word: str
        for word in f:
            kv: list[str] = word.lower().strip().split(' ')
            assert len(kv) == 2

            key: str
            value: str

            key, value = kv
            if key in TSR_dict: # or value in TSR_dict.values():
                print(kv)
                assert value == TSR_dict[key], f"If it is in the dict. it has to have the same TSR spelling:\n {value} != {TSR_dict[key]}"
                continue

            TSR_dict[key] = value

    with open("./dictionary/UNCHANGED.dict", 'w', encoding="utf8") as unf:
        with open("./dictionary/TSR.dict", 'w', encoding="utf8") as tsrf:
            sorter_dict: dict[str, str] = dict(sorted(TSR_dict.items().__iter__()))

            for k, v in sorter_dict.items():
                if k == v:
                    unf.write(f"{k} {v}\n")
                else:
                    tsrf.write(f"{k} {v}\n")

    gen.gen()

if __name__ == '__main__':
    add_new_word()

