import * as wasm from "ct-calculator"

const inputFields = [
    ['Binary', 2],
    ['Decimal', 10],
    ['Hexadecimal', 16]
]


const OPERATION = [["plus", "add"], ["minus", "subtract"]];

let chosenBits = 4
let previousValues = []
let operation = 'add'

let values = {
    right: null,
    left: null,
    result: null
}


function setInput(from) {
    let value = values[from.toLowerCase()]

    if (value === undefined || value === null) {
        return
    }

    document.getElementById('input' + from + 'Binary').value = value.get_bin
    document.getElementById('input' + from + '2C').value = value.get_com
    document.getElementById('input' + from + 'Decimal').value = value.get_signed
    document.getElementById('input' + from + 'Hexadecimal').value = value.get_hex
}

function setResult() {
    let res = values.result.get_value

    if (res === undefined || res === null) {
        return
    }

    // update UI TODO:
    document.getElementById('outputBinary').value = res.get_bin
    document.getElementById('outputHexadecimal').value = res.get_hex
    document.getElementById('unsigned').value = res.get_unsigned
    document.getElementById('signed').value = res.get_signed
}

// update flags on UI
function updateFlags() {
    let { zero, overflow, carry, negative, borrow } = values.result.get_flags

    document.getElementById('flagNegative').innerText = negative ? '1' : '0'
    document.getElementById('flagZero').innerText = zero ? '1' : '0'
    document.getElementById('flagOverflow').innerText = overflow ? '1' : '0'
    document.getElementById('flagCarry').innerText = carry ? '1' : '0'
    document.getElementById('flagBorrow').innerText = borrow ? '1' : '0'
}

function calculateResult() {
    let res = null

    let rawLeft = document.getElementById('inputLeftDecimal').value
    let rawRight = document.getElementById('inputRightDecimal').value

    let left = parseInt(rawLeft, 10)
    let right = parseInt(rawRight, 10)

    console.log(operation)

    switch (operation) {
        case "add":
            res = wasm.add(left, right, chosenBits)
            break
        case "subtract":
            res = wasm.sub(left, right, chosenBits)
            break
        default:
            console.log("No clue how you landed here pall.")
            return
    }

    values.result = res

    updateFlags()
    setResult()
}

function main() {
    // register bit amount event listener
    for (let id = 4; id <= 32; id *= 2) {
        document.getElementById(id + 'bit').addEventListener('click', () => {
            chosenBits = id
            for (let location of ['Left', 'Right']) {
                for (let [type, _] of inputFields) {
                    document.getElementById('input' + location + type).value = ''
                }
                document.getElementById('input' + location + "2C").value = ''
            }
        })
    }

    for (let [sign, op] of OPERATION) {
        document.getElementById(sign).addEventListener('click', () => {
            operation = op
            calculateResult()
        })
    }

    // register listeners on input fields
    for (let [type, base] of inputFields) {
        for (let location of ['Left', 'Right']) {
            document.getElementById('input' + location + type).addEventListener('keyup', (e) => {
                let value = e.currentTarget.valueOf().value

                if (value === '') {
                    return
                }

                let ivalue = parseInt(value, base)

                if (ivalue.toString(2).length > chosenBits) {
                    e.currentTarget.valueOf().value = previousValues[base]
                    return
                }

                try {
                    let result = wasm.format(ivalue, chosenBits)
                    values[location.toLowerCase()] = result
                } catch (e) {
                    console.log("no clue what happend")
                    console.log(e)
                }

                setInput(location)
                calculateResult()
            })
        }
    }
}

main()
