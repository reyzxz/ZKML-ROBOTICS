pragma circom 2.0.0;

component main = CommandCommit();

template CommandCommit() {
    signal input actionHash;
    signal input paramsHash;
    signal output proofHash;

    proofHash <== actionHash + paramsHash;
}
