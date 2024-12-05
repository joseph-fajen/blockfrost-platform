/*

You can generate them from a set of JSON files with something like:

```shell
idx=1; for f in *.json ; do
  symb=$(jq -r '.testCases[0].json.contents.contents.contents.error' "$f" | grep -Eo '([A-Z][a-z]*)+' | head -n 4 | tr '\n' '_' | sed -r 's/(^_+|_+$)//g')
  cbor=$(jq -r '.testCases[0].cbor' "$f")
  numb=$(printf '%04d' "$idx")
  echo '#[tokio::test]'
  echo '#[allow(non_snake_case)]'
  echo 'async fn test_cbor_'"$numb"'_'"$symb"'() {'
  echo '    verify_one("'"$cbor"'").await'
  echo '}'
  echo
  idx=$((idx + 1))
done
```

*/

use super::verify_one;

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0001_ConwayTreasuryValueMismatch_Coin_Coin() {
    verify_one("8182068183051a000de7561a00080fd6").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0002_ConwayMempoolFailure_ConwayGovFailure_ExpirationEpochTooSmall_List() {
    verify_one("8182068282076082038207a0").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0003_ConwayUtxowFailure_MissingTxBodyMetadataHash_AuxiliaryDataHash_AuxiliaryDataHash(
) {
    verify_one(
        "818206818201820558200e13ba83be25492abf84e10545393932480e8ad43dacf8a3d93dff388cce84ed",
    )
    .await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0004_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash() {
    verify_one("81820681820481581c22782faa6bd0c54048b6176eb0cc2f4aa6c56818b3b9075e480e4cbf").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0005_ConwayTxRefScriptsSizeTooBig() {
    verify_one("8182068183060001").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0006_ConwayUtxowFailure_MalformedScriptWitnesses_List() {
    verify_one("8182068182018210d9010280").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0007_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash_ConwayTreasuryValueMismatch() {
    verify_one("81820682820481581cab4f400015b95d3b7c45a285fe08da9e4cc110b06105788819890a7283051a0001abb81a0007fc34").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0008_ConwayCertsFailure_CertFailure_GovCertFailure_ConwayCommitteeHasPreviouslyResigned(
) {
    verify_one("8182068282028201820382038200581cde174ee9f903cd93028d16e1bd0df936ddf2a842f2aa414db0598b6782038302581de0c3a48544970283c379904bf33f5ab2b8e1f6fac902a14ddcd18d2bb900").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0009_ConwayTreasuryValueMismatch_Coin_Coin_ConwayTreasuryValueMismatch() {
    verify_one("8182068283051a0006144d1a0007f68283051a000ab04e1a0003c428").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0010_ConwayMempoolFailure() {
    verify_one("8182068182076162").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0011_ConwayCertsFailure_WithdrawalsNotInRewardsCERTS_List_RewardAccount() {
    verify_one("8182068182028200a1581de180c1af75f8e788b08272ee30e8d87bc776e4bfc47adb0da175bf26ac1a000212eb").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0012_ConwayGovFailure_DisallowedProposalDuringBootstrap_ProposalProcedure_ProcDeposit(
) {
    verify_one("818206828203820c841a00043894581de05c60cda4d195859022a5dc288a826c9d413349697e3009dd8163b3358301825820b3a3b00795156a4bd4338afe1d5d1ed55969c088dbc37c134b6346bff9b7210001820b0082783b68747470733a2f2f365859374137397562386e7a755a687a6f73316c4155546a685830416d7a6a715a744837795a4779475a5071694b542e636f6d582049571f726fd12c21b39edd426be658fbc95e5b14cf3b235338573ef9daa1f4c68203830b81581cd3e73bd6d14a851a663cd925ff72ebacad31da7a6cdbddd623087b9780").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0013_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash() {
    verify_one("81820681820481581c3f784466c9efbcbc998ee0121bca9c4dc03dc37b756ed522e4d46ea6").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0014_ConwayTreasuryValueMismatch_Coin_Coin_ConwayTxRefScriptsSizeTooBig() {
    verify_one("8182068283051a00059a381a000393c683060101").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0015_ConwayMempoolFailure() {
    verify_one("8182068182076160").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0016_ConwayTxRefScriptsSizeTooBig_ConwayTreasuryValueMismatch_Coin_Coin() {
    verify_one("818206828306012083051a000218a31a000594fd").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0017_ConwayTreasuryValueMismatch_Coin_Coin_ConwayWdrlNotDelegatedToDRep() {
    verify_one("8182068283051a000d755a1a000438a6820482581cf88d2b1e7a199cc2791ecd58b2dba509ebb3213f8a45d76fe6565acf581c9d7fbbc29cea56a0b2cdb7c98f7ebef884ad509b5fecfbdc5d9e0a78").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0018_ConwayTxRefScriptsSizeTooBig_ConwayGovFailure_VotersDoNotExist_DRepVoter() {
    verify_one("81820682830601008203820e828203581cb921639de9f45aa695050cbb0746979c85e8897e33831a34387222d48204581cd0d83eac6aea2ae34f39c627996457b2eed18ad3283a62b0dbec896a").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0019_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash_KeyHash() {
    verify_one("81820682820482581cc665e067f9c5af2973d41f470db5e85c1f3495958d9ce9f458117a02581cd16ff9615b0f243f73576a54d7b0ee5fd3f1a827899c50191d3cb9b483060120").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0020_ConwayTreasuryValueMismatch_Coin_Coin_ConwayWdrlNotDelegatedToDRep() {
    verify_one("8182068283051a000aec461a000b26cd820481581c651af173086c113865d55b9abbad7c47aa9569b85f26873ed1b281bd").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0021_ConwayTreasuryValueMismatch_Coin_Coin() {
    verify_one("8182068183051a000b55c61a000921de").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0022_ConwayMempoolFailure_R_ConwayGovFailure_VotingOnExpiredGovAction() {
    verify_one("81820682820761528203820981828204581c5e5b6fe689a2a0b842304c912712328aa704b6c49d15e8b60e6d979882582094259fb315f35d28860159dd35231ce60ee3f99d905f9e6519731a505540bb4e01").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0023_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash_ConwayUtxowFailure() {
    verify_one("81820682820481581cbb05dc8589898474b225a13314de415931c384c1013d90c9b10d0f9c8201830d8158208ccf10dc8526e35d4c21bbfa98507c1c9e58cd7d2483a6c502213c3d5fc2f40981582027e04b9c68972e6dc9e8cf34916c338a00a7665df06eb526f333b7af3e908315").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0024_ConwayGovFailure_ConflictingCommitteeUpdate_List_ConwayGovFailure() {
    verify_one("8182068282038206d901028082038303d901028001").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0025_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash_KeyHash() {
    verify_one("81820681820482581cea98c5db378729e96764fca2d3ca9f2d1973f59e0abecd37833338c0581ceafa36c204c8c0fee37dcbf776c13f31dbe4fd8915cc334beb26e0e9").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0026_ConwayGovFailure_MalformedProposal_HardForkInitiation_SJust() {
    verify_one("818206828203820183018258203d417a35bce157152945acfe78da6938f3ade318bed839ad70d62cd827abf4470082070182076169").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0027_ConwayWdrlNotDelegatedToDRep_KeyHash_KeyHash_KeyHash() {
    verify_one("81820682820482581c16e839e30b01738d1115c3efcf1262fac1797d8bd0089de9353aa106581cb5813031a33030f4922a9232e016e25818eefb2e2ccc450cb11f89ec82076138").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0028_ConwayTreasuryValueMismatch_Coin_Coin() {
    verify_one("8182068183051a0003ec3a1a0003e0c0").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0029_ConwayTreasuryValueMismatch_Coin_Coin_ConwayWdrlNotDelegatedToDRep() {
    verify_one("8182068283051a000bbfe91a000c125a820482581ca9091c6a554fb48870c42fdb0e367476f4795fea34177de56591bbef581c4ad1ed28f5d0590603d941434b07083a8bcd13cec2d5be9a0b5c8529").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0030_ConwayTxRefScriptsSizeTooBig_ConwayGovFailure_DisallowedVotesDuringBootstrap_DRepVoter(
) {
    verify_one("81820682830601208203820d82828203581ca8d4974095d18fd37c4ba1da80a36851b8345ddaf233a7d75f927e2f825820da98f0e22a86a94d6a7e6f0f526959092f1a6ded895287b15dde2b78d84baf3b00828202581c6e56c2c5a5040fcda445ca0a87436e002e540870fad295f6557077bb8258204a184435fe098d9e62e429475b4e337be0acb369c84bbe39baa74740ef6d69d401").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0031_ConwayUtxowFailure_UtxoFailure_InsufficientCollateral_DeltaCoin() {
    verify_one("8182068282018200830c1a000d8a871a000b075682018209d9010280").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0032_ConwayTxRefScriptsSizeTooBig_ConwayGovFailure_ExpirationEpochTooSmall_List()
{
    verify_one("818206828306000082038207a18201581cff5c52bb623f42d8d4a48bc9393011167c82650799fe2057d0ffe17800").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0033_ConwayMempoolFailure_ConwayUtxowFailure_MalformedReferenceScripts_List() {
    verify_one("8182068282076082018211d9010281581cfa0165d3392a8938b5b5a4851d5802f43233ae1f9305b408f39477fd").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0034_ConwayUtxowFailure_InvalidWitnessesUTXOW_VKey_VerKeyEd() {
    verify_one("8182068282018201815820db765eede4a13a462c48279c62d3b614d3c936e765d8086470fff6f277e2d76e83051a000a786f1a000b2866").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0035_ConwayGovFailure_MalformedProposal_NewConstitution_SJust() {
    verify_one("81820682820382018305825820b50c881a532f8e51bf3d1b0297acad3066f93e8fc8e16793b07396c01a9d1ad3008282784068747470733a2f2f764875706f33544d47757179795967523271465a545a6c7845615069775875445a7261464c4a49786c584d753134386a514f30332e636f6d5820c6e1e8abededabeba0fb369afd37876919ff1b72c9bd0731304aa3221905e7bb581cbee75c2524e050e9d9c29e1882acc85241563a11d4f4a15158e1093183060100").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0036_ConwayTreasuryValueMismatch_Coin_Coin() {
    verify_one("8182068183051a0007f26b1a000590a2").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0037_ConwayTreasuryValueMismatch_Coin_Coin_ConwayMempoolFailure() {
    verify_one("8182068283051a000b3e831a000356b2820760").await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0038_ConwayGovFailure_VotersDoNotExist_DRepVoter_KeyHashObj() {
    verify_one(
        "818206828203820e818202581c6405197a2f6592f55ba348f14d540f35caf3a1dedf1d40cd8e474e04820760",
    )
    .await
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_cbor_0039_ConwayUtxowFailure_PPViewHashesDontMatch_SJust_SafeHash() {
    verify_one("818206828201830d815820d15252d17f47ba042f29becbd844d9aabd83b7dafb52bd46589fb10358adb9618158201d97ba11111aee873b749791e741e6f6e9e3d3e7a5e2fc0e532ac6f323a120bf83051a0009c7ac1a000b7da8").await
}
