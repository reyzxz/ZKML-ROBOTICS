pragma circom 2.0.0;

component main = Navigation();

template Navigation() {
    signal input startPos[2];
    signal input endPos[2];
    signal output pathValid;

    pathValid <== (endPos[0] - startPos[0]) * (endPos[1] - startPos[1]);
}
