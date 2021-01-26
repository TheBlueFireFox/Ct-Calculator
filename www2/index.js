import * as wasm from "ct-calculator"

const inputFields = [
    ['Binary', 2],
    ['Decimal', 10],
    ['Hexadecimal', 16]
]

const OPERATION = [["plus", "add"], ["minus", "subtract"]];

var chosenBits = 4
var operation = 'add'

var values = {
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

function setFlags() {
    let { zero, overflow, carry, negative, borrow } = values.result.get_flags

    document.getElementById('flagNegative').innerText = negative ? '1' : '0'
    document.getElementById('flagZero').innerText = zero ? '1' : '0'
    document.getElementById('flagOverflow').innerText = overflow ? '1' : '0'
    document.getElementById('flagCarry').innerText = carry ? '1' : '0'
    document.getElementById('flagBorrow').innerText = borrow ? '1' : '0'
}

function setCondUnsigned() {
    const works = (val) => val ? '1' : '0'
    let { zero, carry } = values.result.get_flags

    document.getElementById("condUnsignedEQ").innerText = works(zero)
    document.getElementById("condUnsignedNE").innerText = works(!zero)

    document.getElementById("condUnsignedHS").innerText = works(carry)
    document.getElementById("condUnsignedLO").innerText = works(!carry)


    document.getElementById("condUnsignedHI").innerText = works(carry && !zero)
    document.getElementById("condUnsignedLS").innerText = works(!carry || zero)
}

function setCondSigned() {
    const works = (val) => val ? '1' : '0'
    let { zero, overflow, negative } = values.result.get_flags

    document.getElementById("condSignedEQ").innerText = works(zero)
    document.getElementById("condSignedNE").innerText = works(!zero)

    document.getElementById("condSignedMI").innerText = works(negative)
    document.getElementById("condSignedPL").innerText = works(!negative)

    document.getElementById("condSignedVS").innerText = works(overflow)
    document.getElementById("condSignedVC").innerText = works(!overflow)

    document.getElementById("condSignedGE").innerText = works(negative == overflow)
    document.getElementById("condSignedLT").innerText = works(negative != overflow)

    document.getElementById("condSignedGT").innerText = works(!zero && (negative === overflow))
    document.getElementById("condSignedLE").innerText = works(zero || (negative !== overflow))
}

function setCond() {
    setCondSigned()
    setCondUnsigned()
}

function calculateResult() {
    let res = null

    let rawLeft = document.getElementById('inputLeftDecimal').value
    let rawRight = document.getElementById('inputRightDecimal').value

    const condition = (name) => name === null || name === undefined || name === ''

    if (condition(rawLeft) || condition(rawRight)) {
        return
    }

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

    setFlags()
    setResult()
    setCond()
}

function resetField(location) {
    for (let [type] of inputFields) {
        document.getElementById('input' + location + type).value = ''
    }
    document.getElementById('input' + location + "2C").value = ''
    resetResults()
}

function resetCondUnsigned() {
    const names = [
        "condUnsignedEQ",
        "condUnsignedNE",
        "condUnsignedHS",
        "condUnsignedLO",
        "condUnsignedHI",
        "condUnsignedLS"
    ]
    for (let elem of names) {
        document.getElementById(elem).innerText = '0'
    }
}

function resetCondSigned() {
    const names = [
        "condSignedEQ",
        "condSignedNE",
        "condSignedMI",
        "condSignedPL",
        "condSignedVS",
        "condSignedVC",
        "condSignedGE",
        "condSignedLT",
        "condSignedGT",
        "condSignedLE"
    ]
    for (let elem of names) {
        document.getElementById(elem).innerText = '0'
    }

}

function resetCond() {
    resetCondUnsigned()
    resetCondSigned()
}

function resetResults() {
    // reset all the result fields
    for (let where of ['outputBinary', 'outputHexadecimal', 'unsigned', 'signed']) {
        document.getElementById(where).value = ''
    }

    // reset all the flags
    for (let type of ['Negative', 'Zero', 'Overflow', 'Carry', 'Borrow']) {
        document.getElementById("flag" + type).innerText = '0'
    }
}

function reset() {
    // reset all the fields
    for (let field of ["Left", "Right"]) {
        resetField(field)
    }
    resetCond()
}

function main() {
    // clean up everything
    reset()

    // register bit amount event listener
    for (let id = 4; id <= 32; id *= 2) {
        document.getElementById(id + 'bit').addEventListener('click', () => {
            chosenBits = id
            reset()
        })
    }

    // register operation listener
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
                    e.currentTarget.valueOf().value = undefined
                    return
                }

                try {
                    let result = wasm.format(ivalue, chosenBits)
                    values[location.toLowerCase()] = result
                } catch (err) {
                    console.log("no clue what happend")
                    console.log(err)
                }

                setInput(location)
                calculateResult()
            })
        }
    }
}

main()
