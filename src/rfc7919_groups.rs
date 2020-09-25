// RFC7918 Supported Group Registry
// https://tools.ietf.org/html/rfc7919#appendix-A

const FFDHE2048_P: &'static str = "FFFFFFFF FFFFFFFF ADF85458 A2BB4A9A AFDC5620 273D3CF1
                D8B9C583 CE2D3695 A9E13641 146433FB CC939DCE 249B3EF9
                7D2FE363 630C75D8 F681B202 AEC4617A D3DF1ED5 D5FD6561
                2433F51F 5F066ED0 85636555 3DED1AF3 B557135E 7F57C935
                984F0C70 E0E68B77 E2A689DA F3EFE872 1DF158A1 36ADE735
                30ACCA4F 483A797A BC0AB182 B324FB61 D108A94B B2C8E3FB
                B96ADAB7 60D7F468 1D4F42A3 DE394DF4 AE56EDE7 6372BB19
                0B07A7C8 EE0A6D70 9E02FCE1 CDF7E2EC C03404CD 28342F61
                9172FE9C E98583FF 8E4F1232 EEF28183 C3FE3B1B 4C6FAD73
                3BB5FCBC 2EC22005 C58EF183 7D1683B2 C6F34A26 C1B2EFFA
                886B4238 61285C97 FFFFFFFF FFFFFFFF";

const FFDHE2048_Q: &'static str = "7FFFFFFF FFFFFFFF D6FC2A2C 515DA54D 57EE2B10 139E9E78
                EC5CE2C1 E7169B4A D4F09B20 8A3219FD E649CEE7 124D9F7C
                BE97F1B1 B1863AEC 7B40D901 576230BD 69EF8F6A EAFEB2B0
                9219FA8F AF833768 42B1B2AA 9EF68D79 DAAB89AF 3FABE49A
                CC278638 707345BB F15344ED 79F7F439 0EF8AC50 9B56F39A
                98566527 A41D3CBD 5E0558C1 59927DB0 E88454A5 D96471FD
                DCB56D5B B06BFA34 0EA7A151 EF1CA6FA 572B76F3 B1B95D8C
                8583D3E4 770536B8 4F017E70 E6FBF176 601A0266 941A17B0
                C8B97F4E 74C2C1FF C7278919 777940C1 E1FF1D8D A637D6B9
                9DDAFE5E 17611002 E2C778C1 BE8B41D9 6379A513 60D977FD
                4435A11C 30942E4B FFFFFFFF FFFFFFFF";

const FFDHE3072_P: &'static str = "FFFFFFFF FFFFFFFF ADF85458 A2BB4A9A AFDC5620 273D3CF1
                D8B9C583 CE2D3695 A9E13641 146433FB CC939DCE 249B3EF9
                7D2FE363 630C75D8 F681B202 AEC4617A D3DF1ED5 D5FD6561
                2433F51F 5F066ED0 85636555 3DED1AF3 B557135E 7F57C935
                984F0C70 E0E68B77 E2A689DA F3EFE872 1DF158A1 36ADE735
                30ACCA4F 483A797A BC0AB182 B324FB61 D108A94B B2C8E3FB
                B96ADAB7 60D7F468 1D4F42A3 DE394DF4 AE56EDE7 6372BB19
                0B07A7C8 EE0A6D70 9E02FCE1 CDF7E2EC C03404CD 28342F61
                9172FE9C E98583FF 8E4F1232 EEF28183 C3FE3B1B 4C6FAD73
                3BB5FCBC 2EC22005 C58EF183 7D1683B2 C6F34A26 C1B2EFFA
                886B4238 611FCFDC DE355B3B 6519035B BC34F4DE F99C0238
                61B46FC9 D6E6C907 7AD91D26 91F7F7EE 598CB0FA C186D91C
                AEFE1309 85139270 B4130C93 BC437944 F4FD4452 E2D74DD3
                64F2E21E 71F54BFF 5CAE82AB 9C9DF69E E86D2BC5 22363A0D
                ABC52197 9B0DEADA 1DBF9A42 D5C4484E 0ABCD06B FA53DDEF
                3C1B20EE 3FD59D7C 25E41D2B 66C62E37 FFFFFFFF FFFFFFFF";

const FFDHE3072_Q: &'static str = "7FFFFFFF FFFFFFFF D6FC2A2C 515DA54D 57EE2B10 139E9E78
                EC5CE2C1 E7169B4A D4F09B20 8A3219FD E649CEE7 124D9F7C
                BE97F1B1 B1863AEC 7B40D901 576230BD 69EF8F6A EAFEB2B0
                9219FA8F AF833768 42B1B2AA 9EF68D79 DAAB89AF 3FABE49A
                CC278638 707345BB F15344ED 79F7F439 0EF8AC50 9B56F39A
                98566527 A41D3CBD 5E0558C1 59927DB0 E88454A5 D96471FD
                DCB56D5B B06BFA34 0EA7A151 EF1CA6FA 572B76F3 B1B95D8C
                8583D3E4 770536B8 4F017E70 E6FBF176 601A0266 941A17B0
                C8B97F4E 74C2C1FF C7278919 777940C1 E1FF1D8D A637D6B9
                9DDAFE5E 17611002 E2C778C1 BE8B41D9 6379A513 60D977FD
                4435A11C 308FE7EE 6F1AAD9D B28C81AD DE1A7A6F 7CCE011C
                30DA37E4 EB736483 BD6C8E93 48FBFBF7 2CC6587D 60C36C8E
                577F0984 C289C938 5A098649 DE21BCA2 7A7EA229 716BA6E9
                B279710F 38FAA5FF AE574155 CE4EFB4F 743695E2 911B1D06
                D5E290CB CD86F56D 0EDFCD21 6AE22427 055E6835 FD29EEF7
                9E0D9077 1FEACEBE 12F20E95 B363171B FFFFFFFF FFFFFFFF";

const FFDHE4096_P: &'static str = "FFFFFFFF FFFFFFFF ADF85458 A2BB4A9A AFDC5620 273D3CF1
D8B9C583 CE2D3695 A9E13641 146433FB CC939DCE 249B3EF9
7D2FE363 630C75D8 F681B202 AEC4617A D3DF1ED5 D5FD6561
2433F51F 5F066ED0 85636555 3DED1AF3 B557135E 7F57C935
984F0C70 E0E68B77 E2A689DA F3EFE872 1DF158A1 36ADE735
30ACCA4F 483A797A BC0AB182 B324FB61 D108A94B B2C8E3FB
B96ADAB7 60D7F468 1D4F42A3 DE394DF4 AE56EDE7 6372BB19
0B07A7C8 EE0A6D70 9E02FCE1 CDF7E2EC C03404CD 28342F61
9172FE9C E98583FF 8E4F1232 EEF28183 C3FE3B1B 4C6FAD73
3BB5FCBC 2EC22005 C58EF183 7D1683B2 C6F34A26 C1B2EFFA
886B4238 611FCFDC DE355B3B 6519035B BC34F4DE F99C0238
61B46FC9 D6E6C907 7AD91D26 91F7F7EE 598CB0FA C186D91C
AEFE1309 85139270 B4130C93 BC437944 F4FD4452 E2D74DD3
64F2E21E 71F54BFF 5CAE82AB 9C9DF69E E86D2BC5 22363A0D
ABC52197 9B0DEADA 1DBF9A42 D5C4484E 0ABCD06B FA53DDEF
3C1B20EE 3FD59D7C 25E41D2B 669E1EF1 6E6F52C3 164DF4FB
7930E9E4 E58857B6 AC7D5F42 D69F6D18 7763CF1D 55034004
87F55BA5 7E31CC7A 7135C886 EFB4318A ED6A1E01 2D9E6832
A907600A 918130C4 6DC778F9 71AD0038 092999A3 33CB8B7A
1A1DB93D 7140003C 2A4ECEA9 F98D0ACC 0A8291CD CEC97DCF
8EC9B55A 7F88A46B 4DB5A851 F44182E1 C68A007E 5E655F6A
FFFFFFFF FFFFFFFF";

const FFDHE4096_Q: &'static str = "7FFFFFFF FFFFFFFF D6FC2A2C 515DA54D 57EE2B10 139E9E78
EC5CE2C1 E7169B4A D4F09B20 8A3219FD E649CEE7 124D9F7C
BE97F1B1 B1863AEC 7B40D901 576230BD 69EF8F6A EAFEB2B0
9219FA8F AF833768 42B1B2AA 9EF68D79 DAAB89AF 3FABE49A
CC278638 707345BB F15344ED 79F7F439 0EF8AC50 9B56F39A
98566527 A41D3CBD 5E0558C1 59927DB0 E88454A5 D96471FD
DCB56D5B B06BFA34 0EA7A151 EF1CA6FA 572B76F3 B1B95D8C
8583D3E4 770536B8 4F017E70 E6FBF176 601A0266 941A17B0
C8B97F4E 74C2C1FF C7278919 777940C1 E1FF1D8D A637D6B9
9DDAFE5E 17611002 E2C778C1 BE8B41D9 6379A513 60D977FD
4435A11C 308FE7EE 6F1AAD9D B28C81AD DE1A7A6F 7CCE011C
30DA37E4 EB736483 BD6C8E93 48FBFBF7 2CC6587D 60C36C8E
577F0984 C289C938 5A098649 DE21BCA2 7A7EA229 716BA6E9
B279710F 38FAA5FF AE574155 CE4EFB4F 743695E2 911B1D06
D5E290CB CD86F56D 0EDFCD21 6AE22427 055E6835 FD29EEF7
9E0D9077 1FEACEBE 12F20E95 B34F0F78 B737A961 8B26FA7D
BC9874F2 72C42BDB 563EAFA1 6B4FB68C 3BB1E78E AA81A002
43FAADD2 BF18E63D 389AE443 77DA18C5 76B50F00 96CF3419
5483B005 48C09862 36E3BC7C B8D6801C 0494CCD1 99E5C5BD
0D0EDC9E B8A0001E 15276754 FCC68566 054148E6 E764BEE7
C764DAAD 3FC45235 A6DAD428 FA20C170 E345003F 2F32AFB5
7FFFFFFF FFFFFFFF";

const FFDHE6144_P: &'static str = "FFFFFFFF FFFFFFFF ADF85458 A2BB4A9A AFDC5620 273D3CF1
                D8B9C583 CE2D3695 A9E13641 146433FB CC939DCE 249B3EF9
                7D2FE363 630C75D8 F681B202 AEC4617A D3DF1ED5 D5FD6561
                2433F51F 5F066ED0 85636555 3DED1AF3 B557135E 7F57C935
                984F0C70 E0E68B77 E2A689DA F3EFE872 1DF158A1 36ADE735
                30ACCA4F 483A797A BC0AB182 B324FB61 D108A94B B2C8E3FB
                B96ADAB7 60D7F468 1D4F42A3 DE394DF4 AE56EDE7 6372BB19
                0B07A7C8 EE0A6D70 9E02FCE1 CDF7E2EC C03404CD 28342F61
                9172FE9C E98583FF 8E4F1232 EEF28183 C3FE3B1B 4C6FAD73
                3BB5FCBC 2EC22005 C58EF183 7D1683B2 C6F34A26 C1B2EFFA
                886B4238 611FCFDC DE355B3B 6519035B BC34F4DE F99C0238
                61B46FC9 D6E6C907 7AD91D26 91F7F7EE 598CB0FA C186D91C
                AEFE1309 85139270 B4130C93 BC437944 F4FD4452 E2D74DD3
                64F2E21E 71F54BFF 5CAE82AB 9C9DF69E E86D2BC5 22363A0D
                ABC52197 9B0DEADA 1DBF9A42 D5C4484E 0ABCD06B FA53DDEF
                3C1B20EE 3FD59D7C 25E41D2B 669E1EF1 6E6F52C3 164DF4FB
                7930E9E4 E58857B6 AC7D5F42 D69F6D18 7763CF1D 55034004
                87F55BA5 7E31CC7A 7135C886 EFB4318A ED6A1E01 2D9E6832
                A907600A 918130C4 6DC778F9 71AD0038 092999A3 33CB8B7A
                1A1DB93D 7140003C 2A4ECEA9 F98D0ACC 0A8291CD CEC97DCF
                8EC9B55A 7F88A46B 4DB5A851 F44182E1 C68A007E 5E0DD902
                0BFD64B6 45036C7A 4E677D2C 38532A3A 23BA4442 CAF53EA6
                3BB45432 9B7624C8 917BDD64 B1C0FD4C B38E8C33 4C701C3A
                CDAD0657 FCCFEC71 9B1F5C3E 4E46041F 388147FB 4CFDB477
                A52471F7 A9A96910 B855322E DB6340D8 A00EF092 350511E3
                0ABEC1FF F9E3A26E 7FB29F8C 183023C3 587E38DA 0077D9B4
                763E4E4B 94B2BBC1 94C6651E 77CAF992 EEAAC023 2A281BF6
                B3A739C1 22611682 0AE8DB58 47A67CBE F9C9091B 462D538C
                D72B0374 6AE77F5E 62292C31 1562A846 505DC82D B854338A
                E49F5235 C95B9117 8CCF2DD5 CACEF403 EC9D1810 C6272B04
                5B3B71F9 DC6B80D6 3FDD4A8E 9ADB1E69 62A69526 D43161C1
                A41D570D 7938DAD4 A40E329C D0E40E65 FFFFFFFF FFFFFFFF";

const FFDHE6144_Q: &'static str = "7FFFFFFF FFFFFFFF D6FC2A2C 515DA54D 57EE2B10 139E9E78
                EC5CE2C1 E7169B4A D4F09B20 8A3219FD E649CEE7 124D9F7C
                BE97F1B1 B1863AEC 7B40D901 576230BD 69EF8F6A EAFEB2B0
                9219FA8F AF833768 42B1B2AA 9EF68D79 DAAB89AF 3FABE49A
                CC278638 707345BB F15344ED 79F7F439 0EF8AC50 9B56F39A
                98566527 A41D3CBD 5E0558C1 59927DB0 E88454A5 D96471FD
                DCB56D5B B06BFA34 0EA7A151 EF1CA6FA 572B76F3 B1B95D8C
                8583D3E4 770536B8 4F017E70 E6FBF176 601A0266 941A17B0
                C8B97F4E 74C2C1FF C7278919 777940C1 E1FF1D8D A637D6B9
                9DDAFE5E 17611002 E2C778C1 BE8B41D9 6379A513 60D977FD
                4435A11C 308FE7EE 6F1AAD9D B28C81AD DE1A7A6F 7CCE011C
                30DA37E4 EB736483 BD6C8E93 48FBFBF7 2CC6587D 60C36C8E
                577F0984 C289C938 5A098649 DE21BCA2 7A7EA229 716BA6E9
                B279710F 38FAA5FF AE574155 CE4EFB4F 743695E2 911B1D06
                D5E290CB CD86F56D 0EDFCD21 6AE22427 055E6835 FD29EEF7
                9E0D9077 1FEACEBE 12F20E95 B34F0F78 B737A961 8B26FA7D
                BC9874F2 72C42BDB 563EAFA1 6B4FB68C 3BB1E78E AA81A002
                43FAADD2 BF18E63D 389AE443 77DA18C5 76B50F00 96CF3419
                5483B005 48C09862 36E3BC7C B8D6801C 0494CCD1 99E5C5BD
                0D0EDC9E B8A0001E 15276754 FCC68566 054148E6 E764BEE7
                C764DAAD 3FC45235 A6DAD428 FA20C170 E345003F 2F06EC81
                05FEB25B 2281B63D 2733BE96 1C29951D 11DD2221 657A9F53
                1DDA2A19 4DBB1264 48BDEEB2 58E07EA6 59C74619 A6380E1D
                66D6832B FE67F638 CD8FAE1F 2723020F 9C40A3FD A67EDA3B
                D29238FB D4D4B488 5C2A9917 6DB1A06C 50077849 1A8288F1
                855F60FF FCF1D137 3FD94FC6 0C1811E1 AC3F1C6D 003BECDA
                3B1F2725 CA595DE0 CA63328F 3BE57CC9 77556011 95140DFB
                59D39CE0 91308B41 05746DAC 23D33E5F 7CE4848D A316A9C6
                6B9581BA 3573BFAF 31149618 8AB15423 282EE416 DC2A19C5
                724FA91A E4ADC88B C66796EA E5677A01 F64E8C08 63139582
                2D9DB8FC EE35C06B 1FEEA547 4D6D8F34 B1534A93 6A18B0E0
                D20EAB86 BC9C6D6A 5207194E 68720732 FFFFFFFF FFFFFFFF";

const FFDHE8192_P: &'static str = "FFFFFFFF FFFFFFFF ADF85458 A2BB4A9A AFDC5620 273D3CF1
D8B9C583 CE2D3695 A9E13641 146433FB CC939DCE 249B3EF9
7D2FE363 630C75D8 F681B202 AEC4617A D3DF1ED5 D5FD6561
2433F51F 5F066ED0 85636555 3DED1AF3 B557135E 7F57C935
984F0C70 E0E68B77 E2A689DA F3EFE872 1DF158A1 36ADE735
30ACCA4F 483A797A BC0AB182 B324FB61 D108A94B B2C8E3FB
B96ADAB7 60D7F468 1D4F42A3 DE394DF4 AE56EDE7 6372BB19
0B07A7C8 EE0A6D70 9E02FCE1 CDF7E2EC C03404CD 28342F61
9172FE9C E98583FF 8E4F1232 EEF28183 C3FE3B1B 4C6FAD73
3BB5FCBC 2EC22005 C58EF183 7D1683B2 C6F34A26 C1B2EFFA
886B4238 611FCFDC DE355B3B 6519035B BC34F4DE F99C0238
61B46FC9 D6E6C907 7AD91D26 91F7F7EE 598CB0FA C186D91C
AEFE1309 85139270 B4130C93 BC437944 F4FD4452 E2D74DD3
64F2E21E 71F54BFF 5CAE82AB 9C9DF69E E86D2BC5 22363A0D
ABC52197 9B0DEADA 1DBF9A42 D5C4484E 0ABCD06B FA53DDEF
3C1B20EE 3FD59D7C 25E41D2B 669E1EF1 6E6F52C3 164DF4FB
7930E9E4 E58857B6 AC7D5F42 D69F6D18 7763CF1D 55034004
87F55BA5 7E31CC7A 7135C886 EFB4318A ED6A1E01 2D9E6832
A907600A 918130C4 6DC778F9 71AD0038 092999A3 33CB8B7A
1A1DB93D 7140003C 2A4ECEA9 F98D0ACC 0A8291CD CEC97DCF
8EC9B55A 7F88A46B 4DB5A851 F44182E1 C68A007E 5E0DD902
0BFD64B6 45036C7A 4E677D2C 38532A3A 23BA4442 CAF53EA6
3BB45432 9B7624C8 917BDD64 B1C0FD4C B38E8C33 4C701C3A
CDAD0657 FCCFEC71 9B1F5C3E 4E46041F 388147FB 4CFDB477
A52471F7 A9A96910 B855322E DB6340D8 A00EF092 350511E3
0ABEC1FF F9E3A26E 7FB29F8C 183023C3 587E38DA 0077D9B4
763E4E4B 94B2BBC1 94C6651E 77CAF992 EEAAC023 2A281BF6
B3A739C1 22611682 0AE8DB58 47A67CBE F9C9091B 462D538C
D72B0374 6AE77F5E 62292C31 1562A846 505DC82D B854338A
E49F5235 C95B9117 8CCF2DD5 CACEF403 EC9D1810 C6272B04
5B3B71F9 DC6B80D6 3FDD4A8E 9ADB1E69 62A69526 D43161C1
A41D570D 7938DAD4 A40E329C CFF46AAA 36AD004C F600C838
1E425A31 D951AE64 FDB23FCE C9509D43 687FEB69 EDD1CC5E
0B8CC3BD F64B10EF 86B63142 A3AB8829 555B2F74 7C932665
CB2C0F1C C01BD702 29388839 D2AF05E4 54504AC7 8B758282
2846C0BA 35C35F5C 59160CC0 46FD8251 541FC68C 9C86B022
BB709987 6A460E74 51A8A931 09703FEE 1C217E6C 3826E52C
51AA691E 0E423CFC 99E9E316 50C1217B 624816CD AD9A95F9
D5B80194 88D9C0A0 A1FE3075 A577E231 83F81D4A 3F2FA457
1EFC8CE0 BA8A4FE8 B6855DFE 72B0A66E DED2FBAB FBE58A30
FAFABE1C 5D71A87E 2F741EF8 C1FE86FE A6BBFDE5 30677F0D
97D11D49 F7A8443D 0822E506 A9F4614E 011E2A94 838FF88C
D68C8BB7 C5C6424C FFFFFFFF FFFFFFFF";

const FFDHE8192_Q: &'static str = "7FFFFFFF FFFFFFFF D6FC2A2C 515DA54D 57EE2B10 139E9E78
EC5CE2C1 E7169B4A D4F09B20 8A3219FD E649CEE7 124D9F7C
BE97F1B1 B1863AEC 7B40D901 576230BD 69EF8F6A EAFEB2B0
9219FA8F AF833768 42B1B2AA 9EF68D79 DAAB89AF 3FABE49A
CC278638 707345BB F15344ED 79F7F439 0EF8AC50 9B56F39A
98566527 A41D3CBD 5E0558C1 59927DB0 E88454A5 D96471FD
DCB56D5B B06BFA34 0EA7A151 EF1CA6FA 572B76F3 B1B95D8C
8583D3E4 770536B8 4F017E70 E6FBF176 601A0266 941A17B0
C8B97F4E 74C2C1FF C7278919 777940C1 E1FF1D8D A637D6B9
9DDAFE5E 17611002 E2C778C1 BE8B41D9 6379A513 60D977FD
4435A11C 308FE7EE 6F1AAD9D B28C81AD DE1A7A6F 7CCE011C
30DA37E4 EB736483 BD6C8E93 48FBFBF7 2CC6587D 60C36C8E
577F0984 C289C938 5A098649 DE21BCA2 7A7EA229 716BA6E9
B279710F 38FAA5FF AE574155 CE4EFB4F 743695E2 911B1D06
D5E290CB CD86F56D 0EDFCD21 6AE22427 055E6835 FD29EEF7
9E0D9077 1FEACEBE 12F20E95 B34F0F78 B737A961 8B26FA7D
BC9874F2 72C42BDB 563EAFA1 6B4FB68C 3BB1E78E AA81A002
43FAADD2 BF18E63D 389AE443 77DA18C5 76B50F00 96CF3419
5483B005 48C09862 36E3BC7C B8D6801C 0494CCD1 99E5C5BD
0D0EDC9E B8A0001E 15276754 FCC68566 054148E6 E764BEE7
C764DAAD 3FC45235 A6DAD428 FA20C170 E345003F 2F06EC81
05FEB25B 2281B63D 2733BE96 1C29951D 11DD2221 657A9F53
1DDA2A19 4DBB1264 48BDEEB2 58E07EA6 59C74619 A6380E1D
66D6832B FE67F638 CD8FAE1F 2723020F 9C40A3FD A67EDA3B
D29238FB D4D4B488 5C2A9917 6DB1A06C 50077849 1A8288F1
855F60FF FCF1D137 3FD94FC6 0C1811E1 AC3F1C6D 003BECDA
3B1F2725 CA595DE0 CA63328F 3BE57CC9 77556011 95140DFB
59D39CE0 91308B41 05746DAC 23D33E5F 7CE4848D A316A9C6
6B9581BA 3573BFAF 31149618 8AB15423 282EE416 DC2A19C5
724FA91A E4ADC88B C66796EA E5677A01 F64E8C08 63139582
2D9DB8FC EE35C06B 1FEEA547 4D6D8F34 B1534A93 6A18B0E0
D20EAB86 BC9C6D6A 5207194E 67FA3555 1B568026 7B00641C
0F212D18 ECA8D732 7ED91FE7 64A84EA1 B43FF5B4 F6E8E62F
05C661DE FB258877 C35B18A1 51D5C414 AAAD97BA 3E499332
E596078E 600DEB81 149C441C E95782F2 2A282563 C5BAC141
1423605D 1AE1AFAE 2C8B0660 237EC128 AA0FE346 4E435811
5DB84CC3 B523073A 28D45498 84B81FF7 0E10BF36 1C137296
28D5348F 07211E7E 4CF4F18B 286090BD B1240B66 D6CD4AFC
EADC00CA 446CE050 50FF183A D2BBF118 C1FC0EA5 1F97D22B
8F7E4670 5D4527F4 5B42AEFF 39585337 6F697DD5 FDF2C518
7D7D5F0E 2EB8D43F 17BA0F7C 60FF437F 535DFEF2 9833BF86
CBE88EA4 FBD4221E 84117283 54FA30A7 008F154A 41C7FC46
6B4645DB E2E32126 7FFFFFFF FFFFFFFF";

pub enum SupportedGroups {
    FFDHE2048,
    FFDHE3072,
    FFDHE4096,
    FFDHE6144,
    FFDHE8192,
}

pub struct SRG {
    p: &'static str,
    q: &'static str,
}

impl SRG {
    pub fn new(group_id: SupportedGroups) -> Self {
        let (p, q) = match group_id {
            SupportedGroups::FFDHE2048 => (FFDHE2048_P, FFDHE2048_Q),
            SupportedGroups::FFDHE3072 => (FFDHE3072_P, FFDHE3072_Q),
            SupportedGroups::FFDHE4096 => (FFDHE4096_P, FFDHE4096_Q),
            SupportedGroups::FFDHE6144 => (FFDHE6144_P, FFDHE6144_Q),
            SupportedGroups::FFDHE8192 => (FFDHE8192_P, FFDHE8192_Q),
        };
        SRG { p, q }
    }

    pub fn p(group_id: SupportedGroups) -> &'static str {
        let p = match group_id {
            SupportedGroups::FFDHE2048 => FFDHE2048_P,
            SupportedGroups::FFDHE3072 => FFDHE3072_P,
            SupportedGroups::FFDHE4096 => FFDHE4096_P,
            SupportedGroups::FFDHE6144 => FFDHE6144_P,
            SupportedGroups::FFDHE8192 => FFDHE8192_P,
        };
        p
    }

    pub fn q(group_id: SupportedGroups) -> &'static str {
        let q = match group_id {
            SupportedGroups::FFDHE2048 => FFDHE2048_Q,
            SupportedGroups::FFDHE3072 => FFDHE3072_Q,
            SupportedGroups::FFDHE4096 => FFDHE4096_Q,
            SupportedGroups::FFDHE6144 => FFDHE6144_Q,
            SupportedGroups::FFDHE8192 => FFDHE8192_Q,
        };
        q
    }
}
