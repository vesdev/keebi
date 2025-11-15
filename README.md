# keebi

running scripts ``keebi <script> <args..>

### examples

``~/.config/keebi/simple.hi``
```
import key from "keebi"

key("a", "click")
```


``~/.config/keebi/spam.hi``
```
import arg, text, sleep from "keebi"

let i = 0

loop {
  if i == parse_int(arg(1)) { break }

  text(format("{a}\n", {a: arg(0)}))

  i += 1

  sleep(1.0)
}
```
