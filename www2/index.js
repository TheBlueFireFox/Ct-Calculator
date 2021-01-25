import * as wasm from "ct-calculator"

const inputFields = [
    ['Binary', 2],
    ['Decimal', 10],
    ['Hexadecimal', 16]
]

let chosenBits = 4
let previousValues = []
let operation = 'add'

let values = {
    right: null,
    left: null,
    result: null
}

// const OPERATIONS = ["add", "subtract"]

// update flags on UI
function updateFlags({ zero, overflow, carry, negative }) {
    document.getElementById('flagNegative').innerText = negative ? '1' : '0'
    document.getElementById('flagZero').innerText = zero ? '1' : '0'
    document.getElementById('flagOverflow').innerText = overflow ? '1' : '0'
    document.getElementById('flagCarry').innerText = carry ? '1' : '0'
    document.getElementById('flagBorrow').innerText = !carry ? '1' : '0'
}


function setInput(from) {
    let value = values[from]

    if (value === undefined || value === null) {
        return
    }

        document.getElementById('input' + from + 'Binary').value = value.
        document.getElementById('input' + from + 'Decimal').value = value.
        document.getElementById('input' + from + 'Hexadecimal').value = value.
}

function setResult() {
    let res = values.res


    // update UI TODO:
    document.getElementById('outputBinary').value = binaryOutput
    document.getElementById('outputHexadecimal').value = hexadecimalOutput.toUpperCase()
    document.getElementById('unsigned').value = parsedDecimal.toString(10)
    document.getElementById('signed').value = parsedResult.toString(10)
}

function calculateResult() {

    let res
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

    updateFlags(res.get_flags)

}

function main() {
    // register bit amount event listener
    for (let id = 4; id <= 32; id *= 2) {
        document.getElementById(id + 'bit').addEventListener('click', () => {
            chosenBits = id
            for (let [type, _] of inputFields) {
                for (let location of ['Left', 'Right']) {
                    document.getElementById('input' + location + type).value = ''
                }
                // document.getElementById('input' + location + "2C").value = ''
            }
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
                } catch {
                }

                setInputs(location.toLowerCase())
                calculateResult()
            })
        }
    }
}

main()
