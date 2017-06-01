function squareRoot(s, e=0.00000001) {
    var x=0;
    while (x*x < s-e || x*x > s+e) {
        x += e;
    }
    return x;
}

console.log(squareRoot(9.0));
