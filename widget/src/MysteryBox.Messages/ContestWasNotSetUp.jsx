const font = fetch(
  "https://fonts.googleapis.com/css2?family=Kodchasan:wght@700&display=swap"
).body;

if (!font) {
  return <></>;
}

const Wrapper = styled.div`
  position: fixed;
  top: 66px;
  bottom: 0;
  left: 0;
  right: 0;
  width: 100%;
`;

const WrapperStars = styled.div`
  height: 100%;
  background: radial-gradient(ellipse at bottom, #1b2735 0%, #090a0f 100%);
  overflow: hidden;
`;

const animationStar = styled.keyframes`
  from {
    transform: translateY(0px);
  }
  to {
    transform: translateY(-2000px);
  }
`;

const SmallStars = styled.div`
  width: 1px;
  height: 1px;
  background: transparent;
  box-shadow: 327px 1250px #fff, 203px 841px #fff, 99px 1520px #fff,
    1226px 659px #fff, 1240px 1226px #fff, 927px 151px #fff, 1470px 755px #fff,
    1560px 1590px #fff, 1368px 1144px #fff, 1988px 1641px #fff,
    779px 1277px #fff, 1034px 170px #fff, 1647px 1653px #fff, 877px 467px #fff,
    775px 180px #fff, 189px 21px #fff, 1733px 1786px #fff, 957px 511px #fff,
    1062px 1920px #fff, 399px 1371px #fff, 1386px 1638px #fff, 1216px 351px #fff,
    1286px 49px #fff, 477px 1227px #fff, 1507px 837px #fff, 1377px 1162px #fff,
    1072px 553px #fff, 1049px 861px #fff, 916px 122px #fff, 980px 1071px #fff,
    1345px 1699px #fff, 60px 1999px #fff, 1125px 916px #fff, 270px 1560px #fff,
    190px 659px #fff, 1388px 1782px #fff, 1113px 1586px #fff, 1006px 1763px #fff,
    13px 7px #fff, 1789px 1915px #fff, 633px 1335px #fff, 598px 1581px #fff,
    1682px 647px #fff, 648px 278px #fff, 856px 1589px #fff, 1194px 312px #fff,
    935px 920px #fff, 2px 651px #fff, 119px 1980px #fff, 1793px 921px #fff,
    1860px 1923px #fff, 752px 404px #fff, 1578px 1442px #fff, 797px 569px #fff,
    817px 1455px #fff, 1749px 1448px #fff, 1019px 1196px #fff,
    1903px 1752px #fff, 1932px 110px #fff, 1246px 1410px #fff, 61px 404px #fff,
    771px 1721px #fff, 1309px 997px #fff, 1996px 380px #fff, 797px 734px #fff,
    63px 1694px #fff, 1262px 1911px #fff, 648px 507px #fff, 1944px 1503px #fff,
    1165px 699px #fff, 195px 1072px #fff, 895px 398px #fff, 1593px 208px #fff,
    397px 831px #fff, 431px 812px #fff, 1456px 748px #fff, 1243px 212px #fff,
    1560px 413px #fff, 1947px 1511px #fff, 1927px 677px #fff, 1361px 1196px #fff,
    1893px 795px #fff, 1821px 495px #fff, 1991px 817px #fff, 1949px 1114px #fff,
    1142px 1856px #fff, 1768px 469px #fff, 1654px 239px #fff, 845px 312px #fff,
    1589px 1936px #fff, 654px 1432px #fff, 276px 993px #fff, 815px 222px #fff,
    443px 1429px #fff, 532px 148px #fff, 1969px 547px #fff, 1714px 255px #fff,
    1579px 1323px #fff, 421px 516px #fff, 464px 770px #fff, 775px 1045px #fff,
    949px 1080px #fff, 123px 1687px #fff, 185px 1895px #fff, 1544px 742px #fff,
    1835px 1002px #fff, 1340px 1220px #fff, 922px 1642px #fff, 1492px 210px #fff,
    1704px 652px #fff, 1156px 1756px #fff, 1352px 1622px #fff,
    1934px 1438px #fff, 1402px 1343px #fff, 1653px 620px #fff, 637px 701px #fff,
    1396px 1016px #fff, 1721px 341px #fff, 1px 1621px #fff, 785px 1280px #fff,
    814px 122px #fff, 1879px 204px #fff, 656px 703px #fff, 1599px 693px #fff,
    252px 1456px #fff, 1377px 277px #fff, 1233px 1377px #fff, 744px 1964px #fff,
    930px 1620px #fff, 1537px 25px #fff, 1618px 436px #fff, 1083px 1276px #fff,
    701px 560px #fff, 1933px 1193px #fff, 1899px 820px #fff, 76px 1269px #fff,
    513px 1474px #fff, 1624px 1662px #fff, 1530px 125px #fff, 1431px 169px #fff,
    1859px 1848px #fff, 824px 887px #fff, 321px 1466px #fff, 1736px 214px #fff,
    1710px 1423px #fff, 1838px 1830px #fff, 1483px 1279px #fff, 240px 173px #fff,
    737px 503px #fff, 489px 1282px #fff, 880px 198px #fff, 226px 420px #fff,
    1220px 1430px #fff, 226px 1318px #fff, 1456px 846px #fff, 1373px 82px #fff,
    852px 268px #fff, 1552px 1654px #fff, 1054px 382px #fff, 137px 1505px #fff,
    938px 1270px #fff, 502px 1022px #fff, 759px 637px #fff, 221px 240px #fff,
    514px 1550px #fff, 1076px 1564px #fff, 1100px 1664px #fff, 1037px 577px #fff,
    643px 132px #fff, 658px 1874px #fff, 1640px 1975px #fff, 1222px 851px #fff,
    1527px 424px #fff, 885px 987px #fff, 35px 1159px #fff, 254px 1405px #fff,
    90px 587px #fff, 1066px 474px #fff, 1789px 1896px #fff, 676px 1817px #fff,
    1024px 1770px #fff, 1987px 1346px #fff, 1663px 697px #fff, 269px 877px #fff,
    1214px 1238px #fff, 769px 293px #fff, 1736px 1446px #fff, 1981px 88px #fff,
    298px 1803px #fff, 836px 1442px #fff, 1298px 527px #fff, 1262px 1703px #fff,
    1819px 1685px #fff, 380px 1304px #fff, 298px 343px #fff, 180px 1561px #fff,
    1669px 778px #fff, 190px 1220px #fff, 236px 1206px #fff, 1179px 1106px #fff,
    1092px 853px #fff, 1181px 465px #fff, 527px 743px #fff, 400px 1308px #fff,
    1418px 271px #fff, 919px 187px #fff, 1780px 1283px #fff, 1681px 718px #fff,
    155px 1921px #fff, 1631px 102px #fff, 1577px 1619px #fff, 1575px 1044px #fff,
    403px 938px #fff, 98px 1064px #fff, 1631px 1763px #fff, 1855px 1531px #fff,
    850px 1188px #fff, 429px 469px #fff, 665px 1663px #fff, 752px 1768px #fff,
    1725px 918px #fff, 1723px 344px #fff, 17px 383px #fff, 1590px 1505px #fff,
    1116px 426px #fff, 1102px 196px #fff, 570px 372px #fff, 308px 876px #fff,
    367px 803px #fff, 58px 1313px #fff, 759px 1058px #fff, 504px 1008px #fff,
    1400px 1499px #fff, 1205px 867px #fff, 135px 1518px #fff, 1184px 1912px #fff,
    654px 1319px #fff, 1366px 871px #fff, 299px 1172px #fff, 1933px 878px #fff,
    31px 193px #fff, 1590px 1718px #fff, 795px 574px #fff, 221px 1320px #fff,
    1894px 652px #fff, 942px 1284px #fff, 1371px 1624px #fff, 664px 690px #fff,
    1768px 332px #fff, 1861px 819px #fff, 914px 948px #fff, 188px 677px #fff,
    582px 1130px #fff, 240px 727px #fff, 523px 931px #fff, 1312px 1124px #fff,
    1841px 510px #fff, 1712px 14px #fff, 1503px 892px #fff, 1408px 1618px #fff,
    1109px 1407px #fff, 271px 19px #fff, 1191px 808px #fff, 134px 238px #fff,
    1028px 1502px #fff, 98px 607px #fff, 1833px 54px #fff, 1945px 927px #fff,
    1594px 1191px #fff, 1126px 1472px #fff, 897px 403px #fff, 333px 1597px #fff,
    1465px 688px #fff, 446px 239px #fff, 1618px 1844px #fff, 643px 614px #fff,
    445px 618px #fff, 49px 989px #fff, 363px 540px #fff, 876px 498px #fff,
    312px 589px #fff, 1022px 1362px #fff, 1021px 1133px #fff, 416px 914px #fff,
    1910px 324px #fff, 239px 899px #fff, 927px 638px #fff, 321px 126px #fff,
    515px 886px #fff, 637px 1690px #fff, 499px 1050px #fff, 142px 1095px #fff,
    166px 560px #fff, 1205px 1376px #fff, 1149px 1677px #fff, 508px 244px #fff,
    683px 929px #fff, 1367px 809px #fff, 229px 1278px #fff, 346px 1869px #fff,
    1313px 1013px #fff, 1479px 685px #fff, 1146px 1268px #fff, 1898px 760px #fff,
    992px 888px #fff, 1521px 291px #fff, 1541px 1220px #fff, 1418px 246px #fff,
    1400px 1551px #fff, 316px 730px #fff, 1581px 642px #fff, 564px 181px #fff,
    377px 950px #fff, 208px 564px #fff, 1230px 1631px #fff, 1531px 321px #fff,
    504px 56px #fff, 997px 1385px #fff, 501px 1197px #fff, 1012px 1850px #fff,
    225px 999px #fff, 372px 348px #fff, 1338px 405px #fff, 1875px 1591px #fff,
    330px 1777px #fff, 673px 1488px #fff, 1615px 1387px #fff, 1347px 271px #fff,
    934px 681px #fff, 718px 1865px #fff, 945px 1711px #fff, 442px 1351px #fff,
    57px 1437px #fff, 7px 1999px #fff, 1992px 478px #fff, 1992px 486px #fff,
    912px 1545px #fff, 1169px 676px #fff, 1225px 625px #fff, 1192px 792px #fff,
    347px 239px #fff, 725px 19px #fff, 436px 347px #fff, 1322px 1021px #fff,
    510px 227px #fff, 1510px 259px #fff, 1525px 1111px #fff, 1491px 1248px #fff,
    1646px 1445px #fff, 6px 352px #fff, 95px 1829px #fff, 1497px 1601px #fff,
    1072px 572px #fff, 657px 765px #fff, 1258px 1898px #fff, 1821px 264px #fff,
    704px 1140px #fff, 373px 441px #fff, 861px 1141px #fff, 1094px 636px #fff,
    141px 1259px #fff, 579px 1699px #fff, 830px 845px #fff, 530px 1235px #fff,
    1419px 447px #fff, 824px 184px #fff, 342px 1384px #fff, 1609px 1563px #fff,
    1507px 977px #fff, 171px 48px #fff, 1577px 1714px #fff, 677px 63px #fff,
    965px 1650px #fff, 257px 1195px #fff, 524px 1561px #fff, 1297px 1503px #fff,
    294px 1257px #fff, 1004px 623px #fff, 941px 245px #fff, 1671px 423px #fff,
    900px 1045px #fff, 1523px 1650px #fff, 1502px 169px #fff, 1329px 496px #fff,
    491px 703px #fff, 1394px 321px #fff, 510px 1267px #fff, 1631px 1031px #fff,
    1346px 1806px #fff, 321px 1527px #fff, 117px 690px #fff, 644px 1036px #fff,
    1543px 1994px #fff, 1884px 1760px #fff, 828px 489px #fff, 1161px 1743px #fff,
    1747px 1833px #fff, 797px 1873px #fff, 1193px 178px #fff, 405px 771px #fff,
    173px 783px #fff, 1636px 1413px #fff, 702px 134px #fff, 140px 658px #fff,
    98px 1516px #fff, 1410px 90px #fff, 1906px 984px #fff, 1159px 1745px #fff,
    1874px 246px #fff, 395px 846px #fff, 768px 1032px #fff, 386px 381px #fff,
    1843px 728px #fff, 528px 847px #fff, 1680px 1px #fff, 1573px 634px #fff,
    1120px 775px #fff, 1832px 1394px #fff, 1368px 1206px #fff, 1607px 132px #fff,
    582px 603px #fff, 573px 572px #fff, 47px 1293px #fff, 1085px 908px #fff,
    1462px 934px #fff, 1364px 969px #fff, 1051px 1255px #fff, 767px 1918px #fff,
    1532px 846px #fff, 1978px 775px #fff, 292px 105px #fff, 477px 255px #fff,
    1510px 1281px #fff, 149px 1896px #fff, 1411px 589px #fff, 586px 1300px #fff,
    1788px 1589px #fff, 1552px 1899px #fff, 1977px 1118px #fff,
    1149px 1480px #fff, 1969px 441px #fff, 1542px 261px #fff, 1549px 1960px #fff,
    23px 135px #fff, 49px 868px #fff, 1118px 1449px #fff, 876px 1924px #fff,
    568px 1648px #fff, 269px 1213px #fff, 55px 89px #fff, 1685px 928px #fff,
    667px 744px #fff, 1593px 639px #fff, 405px 827px #fff, 483px 1481px #fff,
    722px 312px #fff, 1680px 665px #fff, 239px 43px #fff, 918px 1319px #fff,
    408px 788px #fff, 407px 911px #fff, 1344px 291px #fff, 897px 1856px #fff,
    1646px 1778px #fff, 1410px 1093px #fff, 1710px 795px #fff, 1705px 861px #fff,
    707px 1872px #fff, 380px 1511px #fff, 458px 1189px #fff, 1102px 1124px #fff,
    145px 1379px #fff, 1958px 1215px #fff, 983px 487px #fff, 681px 489px #fff,
    598px 641px #fff, 1059px 1247px #fff, 768px 223px #fff, 1734px 291px #fff,
    1580px 1065px #fff, 943px 1222px #fff, 1889px 1713px #fff, 799px 52px #fff,
    1148px 1666px #fff, 1439px 1091px #fff, 245px 1775px #fff,
    1040px 1601px #fff, 45px 1672px #fff, 1859px 326px #fff, 1590px 1864px #fff,
    1425px 343px #fff, 106px 1058px #fff, 1501px 976px #fff, 1181px 1479px #fff,
    687px 1306px #fff, 1428px 1833px #fff, 812px 1883px #fff, 902px 1149px #fff,
    447px 975px #fff, 445px 475px #fff, 1171px 648px #fff, 1666px 1778px #fff,
    89px 258px #fff, 774px 1011px #fff, 1681px 1066px #fff, 1570px 649px #fff,
    442px 1207px #fff, 1033px 1816px #fff, 401px 1166px #fff, 42px 1897px #fff,
    1317px 1347px #fff, 1367px 1630px #fff, 1922px 288px #fff,
    1341px 1076px #fff, 1167px 473px #fff, 869px 907px #fff, 1073px 1541px #fff,
    79px 247px #fff, 1726px 248px #fff, 1273px 1350px #fff, 1885px 1461px #fff,
    164px 853px #fff, 1012px 254px #fff, 1006px 196px #fff, 51px 1083px #fff,
    856px 1813px #fff, 1416px 909px #fff, 653px 1603px #fff, 1567px 1807px #fff,
    1024px 1487px #fff, 1708px 1801px #fff, 1335px 755px #fff,
    1063px 1292px #fff, 998px 1033px #fff, 1421px 778px #fff, 903px 271px #fff,
    1003px 492px #fff, 1266px 1776px #fff, 140px 445px #fff, 765px 687px #fff,
    1042px 747px #fff, 1402px 1193px #fff, 872px 1554px #fff, 1532px 818px #fff,
    1300px 28px #fff, 1549px 1777px #fff, 99px 402px #fff, 865px 1828px #fff,
    1824px 806px #fff, 1650px 752px #fff, 223px 1043px #fff, 882px 307px #fff,
    364px 1988px #fff, 1328px 1992px #fff, 417px 430px #fff, 302px 1782px #fff,
    1728px 881px #fff, 328px 1106px #fff, 646px 1914px #fff, 1061px 41px #fff,
    1550px 142px #fff, 1680px 289px #fff, 686px 1649px #fff, 94px 795px #fff,
    690px 294px #fff, 1867px 170px #fff, 796px 487px #fff, 760px 467px #fff,
    1024px 536px #fff, 1179px 36px #fff, 47px 39px #fff, 711px 177px #fff,
    475px 402px #fff, 522px 1928px #fff, 412px 1474px #fff, 913px 1366px #fff,
    1958px 484px #fff, 608px 1513px #fff, 1169px 1841px #fff, 642px 1424px #fff,
    184px 1814px #fff, 520px 1091px #fff, 1625px 1490px #fff, 445px 288px #fff,
    1244px 1704px #fff, 1580px 495px #fff, 1759px 979px #fff, 1478px 1777px #fff,
    1705px 1818px #fff, 896px 1674px #fff, 957px 1683px #fff, 1853px 426px #fff,
    142px 327px #fff, 380px 524px #fff, 285px 1684px #fff, 672px 323px #fff,
    1999px 1651px #fff, 891px 72px #fff, 653px 1259px #fff, 1979px 555px #fff,
    1398px 33px #fff, 443px 403px #fff, 1567px 1626px #fff, 749px 1147px #fff,
    1270px 1596px #fff, 1799px 243px #fff, 1861px 643px #fff, 1616px 1779px #fff,
    1242px 1957px #fff, 148px 908px #fff, 246px 1359px #fff, 1802px 119px #fff,
    864px 643px #fff, 1321px 1385px #fff, 723px 575px #fff, 993px 219px #fff,
    251px 1046px #fff, 395px 687px #fff, 835px 1020px #fff, 337px 991px #fff,
    1796px 1849px #fff, 1534px 1830px #fff, 1760px 1454px #fff,
    589px 1923px #fff, 1873px 1657px #fff, 1296px 1978px #fff, 1144px 255px #fff,
    1988px 1780px #fff, 1983px 449px #fff, 1355px 113px #fff, 851px 1785px #fff,
    1868px 1338px #fff, 1776px 9px #fff, 1309px 1193px #fff, 1283px 497px #fff,
    967px 459px #fff, 246px 4px #fff, 774px 1126px #fff, 1513px 1813px #fff,
    1201px 978px #fff, 1650px 929px #fff, 1596px 1349px #fff, 903px 1191px #fff,
    1243px 1960px #fff, 1301px 182px #fff, 871px 867px #fff, 633px 1463px #fff,
    1474px 443px #fff, 1391px 357px #fff, 1268px 1127px #fff, 1186px 771px #fff,
    1112px 701px #fff, 475px 332px #fff, 1938px 1465px #fff, 95px 1316px #fff,
    1890px 1902px #fff, 848px 194px #fff, 873px 1149px #fff, 1523px 822px #fff,
    1436px 1856px #fff, 1678px 1530px #fff, 1693px 1027px #fff,
    1344px 204px #fff, 940px 487px #fff, 1744px 438px #fff, 283px 103px #fff,
    922px 1512px #fff, 613px 1070px #fff, 1072px 1656px #fff, 1574px 560px #fff,
    1991px 1652px #fff, 769px 995px #fff, 1757px 1536px #fff, 585px 1015px #fff,
    801px 791px #fff, 252px 554px #fff, 1620px 1074px #fff, 677px 579px #fff,
    986px 1431px #fff, 1484px 320px #fff, 72px 1994px #fff, 562px 1912px #fff,
    1px 1696px #fff, 131px 531px #fff, 1511px 1118px #fff, 662px 1776px #fff,
    1496px 618px #fff, 794px 629px #fff, 74px 1233px #fff, 1107px 754px #fff,
    879px 1762px #fff, 1739px 1285px #fff, 1193px 1414px #fff,
    1807px 1476px #fff, 255px 410px #fff, 699px 1702px #fff, 1703px 47px #fff,
    313px 1533px #fff, 1262px 839px #fff, 188px 1213px #fff;
  animation: ${animationStar} 50s linear infinite;

  &:after {
    content: " ";
    position: absolute;
    top: 2000px;
    width: 1px;
    height: 1px;
    background: transparent;
    box-shadow: 327px 1250px #fff, 203px 841px #fff, 99px 1520px #fff,
      1226px 659px #fff, 1240px 1226px #fff, 927px 151px #fff, 1470px 755px #fff,
      1560px 1590px #fff, 1368px 1144px #fff, 1988px 1641px #fff,
      779px 1277px #fff, 1034px 170px #fff, 1647px 1653px #fff, 877px 467px #fff,
      775px 180px #fff, 189px 21px #fff, 1733px 1786px #fff, 957px 511px #fff,
      1062px 1920px #fff, 399px 1371px #fff, 1386px 1638px #fff,
      1216px 351px #fff, 1286px 49px #fff, 477px 1227px #fff, 1507px 837px #fff,
      1377px 1162px #fff, 1072px 553px #fff, 1049px 861px #fff, 916px 122px #fff,
      980px 1071px #fff, 1345px 1699px #fff, 60px 1999px #fff, 1125px 916px #fff,
      270px 1560px #fff, 190px 659px #fff, 1388px 1782px #fff,
      1113px 1586px #fff, 1006px 1763px #fff, 13px 7px #fff, 1789px 1915px #fff,
      633px 1335px #fff, 598px 1581px #fff, 1682px 647px #fff, 648px 278px #fff,
      856px 1589px #fff, 1194px 312px #fff, 935px 920px #fff, 2px 651px #fff,
      119px 1980px #fff, 1793px 921px #fff, 1860px 1923px #fff, 752px 404px #fff,
      1578px 1442px #fff, 797px 569px #fff, 817px 1455px #fff,
      1749px 1448px #fff, 1019px 1196px #fff, 1903px 1752px #fff,
      1932px 110px #fff, 1246px 1410px #fff, 61px 404px #fff, 771px 1721px #fff,
      1309px 997px #fff, 1996px 380px #fff, 797px 734px #fff, 63px 1694px #fff,
      1262px 1911px #fff, 648px 507px #fff, 1944px 1503px #fff,
      1165px 699px #fff, 195px 1072px #fff, 895px 398px #fff, 1593px 208px #fff,
      397px 831px #fff, 431px 812px #fff, 1456px 748px #fff, 1243px 212px #fff,
      1560px 413px #fff, 1947px 1511px #fff, 1927px 677px #fff,
      1361px 1196px #fff, 1893px 795px #fff, 1821px 495px #fff,
      1991px 817px #fff, 1949px 1114px #fff, 1142px 1856px #fff,
      1768px 469px #fff, 1654px 239px #fff, 845px 312px #fff, 1589px 1936px #fff,
      654px 1432px #fff, 276px 993px #fff, 815px 222px #fff, 443px 1429px #fff,
      532px 148px #fff, 1969px 547px #fff, 1714px 255px #fff, 1579px 1323px #fff,
      421px 516px #fff, 464px 770px #fff, 775px 1045px #fff, 949px 1080px #fff,
      123px 1687px #fff, 185px 1895px #fff, 1544px 742px #fff,
      1835px 1002px #fff, 1340px 1220px #fff, 922px 1642px #fff,
      1492px 210px #fff, 1704px 652px #fff, 1156px 1756px #fff,
      1352px 1622px #fff, 1934px 1438px #fff, 1402px 1343px #fff,
      1653px 620px #fff, 637px 701px #fff, 1396px 1016px #fff, 1721px 341px #fff,
      1px 1621px #fff, 785px 1280px #fff, 814px 122px #fff, 1879px 204px #fff,
      656px 703px #fff, 1599px 693px #fff, 252px 1456px #fff, 1377px 277px #fff,
      1233px 1377px #fff, 744px 1964px #fff, 930px 1620px #fff, 1537px 25px #fff,
      1618px 436px #fff, 1083px 1276px #fff, 701px 560px #fff,
      1933px 1193px #fff, 1899px 820px #fff, 76px 1269px #fff, 513px 1474px #fff,
      1624px 1662px #fff, 1530px 125px #fff, 1431px 169px #fff,
      1859px 1848px #fff, 824px 887px #fff, 321px 1466px #fff, 1736px 214px #fff,
      1710px 1423px #fff, 1838px 1830px #fff, 1483px 1279px #fff,
      240px 173px #fff, 737px 503px #fff, 489px 1282px #fff, 880px 198px #fff,
      226px 420px #fff, 1220px 1430px #fff, 226px 1318px #fff, 1456px 846px #fff,
      1373px 82px #fff, 852px 268px #fff, 1552px 1654px #fff, 1054px 382px #fff,
      137px 1505px #fff, 938px 1270px #fff, 502px 1022px #fff, 759px 637px #fff,
      221px 240px #fff, 514px 1550px #fff, 1076px 1564px #fff,
      1100px 1664px #fff, 1037px 577px #fff, 643px 132px #fff, 658px 1874px #fff,
      1640px 1975px #fff, 1222px 851px #fff, 1527px 424px #fff, 885px 987px #fff,
      35px 1159px #fff, 254px 1405px #fff, 90px 587px #fff, 1066px 474px #fff,
      1789px 1896px #fff, 676px 1817px #fff, 1024px 1770px #fff,
      1987px 1346px #fff, 1663px 697px #fff, 269px 877px #fff,
      1214px 1238px #fff, 769px 293px #fff, 1736px 1446px #fff, 1981px 88px #fff,
      298px 1803px #fff, 836px 1442px #fff, 1298px 527px #fff,
      1262px 1703px #fff, 1819px 1685px #fff, 380px 1304px #fff,
      298px 343px #fff, 180px 1561px #fff, 1669px 778px #fff, 190px 1220px #fff,
      236px 1206px #fff, 1179px 1106px #fff, 1092px 853px #fff,
      1181px 465px #fff, 527px 743px #fff, 400px 1308px #fff, 1418px 271px #fff,
      919px 187px #fff, 1780px 1283px #fff, 1681px 718px #fff, 155px 1921px #fff,
      1631px 102px #fff, 1577px 1619px #fff, 1575px 1044px #fff,
      403px 938px #fff, 98px 1064px #fff, 1631px 1763px #fff, 1855px 1531px #fff,
      850px 1188px #fff, 429px 469px #fff, 665px 1663px #fff, 752px 1768px #fff,
      1725px 918px #fff, 1723px 344px #fff, 17px 383px #fff, 1590px 1505px #fff,
      1116px 426px #fff, 1102px 196px #fff, 570px 372px #fff, 308px 876px #fff,
      367px 803px #fff, 58px 1313px #fff, 759px 1058px #fff, 504px 1008px #fff,
      1400px 1499px #fff, 1205px 867px #fff, 135px 1518px #fff,
      1184px 1912px #fff, 654px 1319px #fff, 1366px 871px #fff,
      299px 1172px #fff, 1933px 878px #fff, 31px 193px #fff, 1590px 1718px #fff,
      795px 574px #fff, 221px 1320px #fff, 1894px 652px #fff, 942px 1284px #fff,
      1371px 1624px #fff, 664px 690px #fff, 1768px 332px #fff, 1861px 819px #fff,
      914px 948px #fff, 188px 677px #fff, 582px 1130px #fff, 240px 727px #fff,
      523px 931px #fff, 1312px 1124px #fff, 1841px 510px #fff, 1712px 14px #fff,
      1503px 892px #fff, 1408px 1618px #fff, 1109px 1407px #fff, 271px 19px #fff,
      1191px 808px #fff, 134px 238px #fff, 1028px 1502px #fff, 98px 607px #fff,
      1833px 54px #fff, 1945px 927px #fff, 1594px 1191px #fff,
      1126px 1472px #fff, 897px 403px #fff, 333px 1597px #fff, 1465px 688px #fff,
      446px 239px #fff, 1618px 1844px #fff, 643px 614px #fff, 445px 618px #fff,
      49px 989px #fff, 363px 540px #fff, 876px 498px #fff, 312px 589px #fff,
      1022px 1362px #fff, 1021px 1133px #fff, 416px 914px #fff,
      1910px 324px #fff, 239px 899px #fff, 927px 638px #fff, 321px 126px #fff,
      515px 886px #fff, 637px 1690px #fff, 499px 1050px #fff, 142px 1095px #fff,
      166px 560px #fff, 1205px 1376px #fff, 1149px 1677px #fff, 508px 244px #fff,
      683px 929px #fff, 1367px 809px #fff, 229px 1278px #fff, 346px 1869px #fff,
      1313px 1013px #fff, 1479px 685px #fff, 1146px 1268px #fff,
      1898px 760px #fff, 992px 888px #fff, 1521px 291px #fff, 1541px 1220px #fff,
      1418px 246px #fff, 1400px 1551px #fff, 316px 730px #fff, 1581px 642px #fff,
      564px 181px #fff, 377px 950px #fff, 208px 564px #fff, 1230px 1631px #fff,
      1531px 321px #fff, 504px 56px #fff, 997px 1385px #fff, 501px 1197px #fff,
      1012px 1850px #fff, 225px 999px #fff, 372px 348px #fff, 1338px 405px #fff,
      1875px 1591px #fff, 330px 1777px #fff, 673px 1488px #fff,
      1615px 1387px #fff, 1347px 271px #fff, 934px 681px #fff, 718px 1865px #fff,
      945px 1711px #fff, 442px 1351px #fff, 57px 1437px #fff, 7px 1999px #fff,
      1992px 478px #fff, 1992px 486px #fff, 912px 1545px #fff, 1169px 676px #fff,
      1225px 625px #fff, 1192px 792px #fff, 347px 239px #fff, 725px 19px #fff,
      436px 347px #fff, 1322px 1021px #fff, 510px 227px #fff, 1510px 259px #fff,
      1525px 1111px #fff, 1491px 1248px #fff, 1646px 1445px #fff, 6px 352px #fff,
      95px 1829px #fff, 1497px 1601px #fff, 1072px 572px #fff, 657px 765px #fff,
      1258px 1898px #fff, 1821px 264px #fff, 704px 1140px #fff, 373px 441px #fff,
      861px 1141px #fff, 1094px 636px #fff, 141px 1259px #fff, 579px 1699px #fff,
      830px 845px #fff, 530px 1235px #fff, 1419px 447px #fff, 824px 184px #fff,
      342px 1384px #fff, 1609px 1563px #fff, 1507px 977px #fff, 171px 48px #fff,
      1577px 1714px #fff, 677px 63px #fff, 965px 1650px #fff, 257px 1195px #fff,
      524px 1561px #fff, 1297px 1503px #fff, 294px 1257px #fff,
      1004px 623px #fff, 941px 245px #fff, 1671px 423px #fff, 900px 1045px #fff,
      1523px 1650px #fff, 1502px 169px #fff, 1329px 496px #fff, 491px 703px #fff,
      1394px 321px #fff, 510px 1267px #fff, 1631px 1031px #fff,
      1346px 1806px #fff, 321px 1527px #fff, 117px 690px #fff, 644px 1036px #fff,
      1543px 1994px #fff, 1884px 1760px #fff, 828px 489px #fff,
      1161px 1743px #fff, 1747px 1833px #fff, 797px 1873px #fff,
      1193px 178px #fff, 405px 771px #fff, 173px 783px #fff, 1636px 1413px #fff,
      702px 134px #fff, 140px 658px #fff, 98px 1516px #fff, 1410px 90px #fff,
      1906px 984px #fff, 1159px 1745px #fff, 1874px 246px #fff, 395px 846px #fff,
      768px 1032px #fff, 386px 381px #fff, 1843px 728px #fff, 528px 847px #fff,
      1680px 1px #fff, 1573px 634px #fff, 1120px 775px #fff, 1832px 1394px #fff,
      1368px 1206px #fff, 1607px 132px #fff, 582px 603px #fff, 573px 572px #fff,
      47px 1293px #fff, 1085px 908px #fff, 1462px 934px #fff, 1364px 969px #fff,
      1051px 1255px #fff, 767px 1918px #fff, 1532px 846px #fff,
      1978px 775px #fff, 292px 105px #fff, 477px 255px #fff, 1510px 1281px #fff,
      149px 1896px #fff, 1411px 589px #fff, 586px 1300px #fff,
      1788px 1589px #fff, 1552px 1899px #fff, 1977px 1118px #fff,
      1149px 1480px #fff, 1969px 441px #fff, 1542px 261px #fff,
      1549px 1960px #fff, 23px 135px #fff, 49px 868px #fff, 1118px 1449px #fff,
      876px 1924px #fff, 568px 1648px #fff, 269px 1213px #fff, 55px 89px #fff,
      1685px 928px #fff, 667px 744px #fff, 1593px 639px #fff, 405px 827px #fff,
      483px 1481px #fff, 722px 312px #fff, 1680px 665px #fff, 239px 43px #fff,
      918px 1319px #fff, 408px 788px #fff, 407px 911px #fff, 1344px 291px #fff,
      897px 1856px #fff, 1646px 1778px #fff, 1410px 1093px #fff,
      1710px 795px #fff, 1705px 861px #fff, 707px 1872px #fff, 380px 1511px #fff,
      458px 1189px #fff, 1102px 1124px #fff, 145px 1379px #fff,
      1958px 1215px #fff, 983px 487px #fff, 681px 489px #fff, 598px 641px #fff,
      1059px 1247px #fff, 768px 223px #fff, 1734px 291px #fff,
      1580px 1065px #fff, 943px 1222px #fff, 1889px 1713px #fff, 799px 52px #fff,
      1148px 1666px #fff, 1439px 1091px #fff, 245px 1775px #fff,
      1040px 1601px #fff, 45px 1672px #fff, 1859px 326px #fff,
      1590px 1864px #fff, 1425px 343px #fff, 106px 1058px #fff,
      1501px 976px #fff, 1181px 1479px #fff, 687px 1306px #fff,
      1428px 1833px #fff, 812px 1883px #fff, 902px 1149px #fff, 447px 975px #fff,
      445px 475px #fff, 1171px 648px #fff, 1666px 1778px #fff, 89px 258px #fff,
      774px 1011px #fff, 1681px 1066px #fff, 1570px 649px #fff,
      442px 1207px #fff, 1033px 1816px #fff, 401px 1166px #fff, 42px 1897px #fff,
      1317px 1347px #fff, 1367px 1630px #fff, 1922px 288px #fff,
      1341px 1076px #fff, 1167px 473px #fff, 869px 907px #fff,
      1073px 1541px #fff, 79px 247px #fff, 1726px 248px #fff, 1273px 1350px #fff,
      1885px 1461px #fff, 164px 853px #fff, 1012px 254px #fff, 1006px 196px #fff,
      51px 1083px #fff, 856px 1813px #fff, 1416px 909px #fff, 653px 1603px #fff,
      1567px 1807px #fff, 1024px 1487px #fff, 1708px 1801px #fff,
      1335px 755px #fff, 1063px 1292px #fff, 998px 1033px #fff,
      1421px 778px #fff, 903px 271px #fff, 1003px 492px #fff, 1266px 1776px #fff,
      140px 445px #fff, 765px 687px #fff, 1042px 747px #fff, 1402px 1193px #fff,
      872px 1554px #fff, 1532px 818px #fff, 1300px 28px #fff, 1549px 1777px #fff,
      99px 402px #fff, 865px 1828px #fff, 1824px 806px #fff, 1650px 752px #fff,
      223px 1043px #fff, 882px 307px #fff, 364px 1988px #fff, 1328px 1992px #fff,
      417px 430px #fff, 302px 1782px #fff, 1728px 881px #fff, 328px 1106px #fff,
      646px 1914px #fff, 1061px 41px #fff, 1550px 142px #fff, 1680px 289px #fff,
      686px 1649px #fff, 94px 795px #fff, 690px 294px #fff, 1867px 170px #fff,
      796px 487px #fff, 760px 467px #fff, 1024px 536px #fff, 1179px 36px #fff,
      47px 39px #fff, 711px 177px #fff, 475px 402px #fff, 522px 1928px #fff,
      412px 1474px #fff, 913px 1366px #fff, 1958px 484px #fff, 608px 1513px #fff,
      1169px 1841px #fff, 642px 1424px #fff, 184px 1814px #fff,
      520px 1091px #fff, 1625px 1490px #fff, 445px 288px #fff,
      1244px 1704px #fff, 1580px 495px #fff, 1759px 979px #fff,
      1478px 1777px #fff, 1705px 1818px #fff, 896px 1674px #fff,
      957px 1683px #fff, 1853px 426px #fff, 142px 327px #fff, 380px 524px #fff,
      285px 1684px #fff, 672px 323px #fff, 1999px 1651px #fff, 891px 72px #fff,
      653px 1259px #fff, 1979px 555px #fff, 1398px 33px #fff, 443px 403px #fff,
      1567px 1626px #fff, 749px 1147px #fff, 1270px 1596px #fff,
      1799px 243px #fff, 1861px 643px #fff, 1616px 1779px #fff,
      1242px 1957px #fff, 148px 908px #fff, 246px 1359px #fff, 1802px 119px #fff,
      864px 643px #fff, 1321px 1385px #fff, 723px 575px #fff, 993px 219px #fff,
      251px 1046px #fff, 395px 687px #fff, 835px 1020px #fff, 337px 991px #fff,
      1796px 1849px #fff, 1534px 1830px #fff, 1760px 1454px #fff,
      589px 1923px #fff, 1873px 1657px #fff, 1296px 1978px #fff,
      1144px 255px #fff, 1988px 1780px #fff, 1983px 449px #fff,
      1355px 113px #fff, 851px 1785px #fff, 1868px 1338px #fff, 1776px 9px #fff,
      1309px 1193px #fff, 1283px 497px #fff, 967px 459px #fff, 246px 4px #fff,
      774px 1126px #fff, 1513px 1813px #fff, 1201px 978px #fff,
      1650px 929px #fff, 1596px 1349px #fff, 903px 1191px #fff,
      1243px 1960px #fff, 1301px 182px #fff, 871px 867px #fff, 633px 1463px #fff,
      1474px 443px #fff, 1391px 357px #fff, 1268px 1127px #fff,
      1186px 771px #fff, 1112px 701px #fff, 475px 332px #fff, 1938px 1465px #fff,
      95px 1316px #fff, 1890px 1902px #fff, 848px 194px #fff, 873px 1149px #fff,
      1523px 822px #fff, 1436px 1856px #fff, 1678px 1530px #fff,
      1693px 1027px #fff, 1344px 204px #fff, 940px 487px #fff, 1744px 438px #fff,
      283px 103px #fff, 922px 1512px #fff, 613px 1070px #fff, 1072px 1656px #fff,
      1574px 560px #fff, 1991px 1652px #fff, 769px 995px #fff,
      1757px 1536px #fff, 585px 1015px #fff, 801px 791px #fff, 252px 554px #fff,
      1620px 1074px #fff, 677px 579px #fff, 986px 1431px #fff, 1484px 320px #fff,
      72px 1994px #fff, 562px 1912px #fff, 1px 1696px #fff, 131px 531px #fff,
      1511px 1118px #fff, 662px 1776px #fff, 1496px 618px #fff, 794px 629px #fff,
      74px 1233px #fff, 1107px 754px #fff, 879px 1762px #fff, 1739px 1285px #fff,
      1193px 1414px #fff, 1807px 1476px #fff, 255px 410px #fff,
      699px 1702px #fff, 1703px 47px #fff, 313px 1533px #fff, 1262px 839px #fff,
      188px 1213px #fff;
  }
`;

const MediumStars = styled.div`
  width: 2px;
  height: 2px;
  background: transparent;
  box-shadow: 368px 598px #fff, 1373px 195px #fff, 279px 85px #fff,
    115px 1514px #fff, 1139px 783px #fff, 797px 816px #fff, 572px 88px #fff,
    490px 789px #fff, 290px 46px #fff, 606px 616px #fff, 1417px 402px #fff,
    1572px 737px #fff, 466px 1146px #fff, 421px 621px #fff, 1863px 1096px #fff,
    423px 1615px #fff, 1915px 790px #fff, 862px 847px #fff, 303px 110px #fff,
    1050px 1531px #fff, 1786px 1072px #fff, 795px 1665px #fff, 1727px 29px #fff,
    1892px 440px #fff, 457px 1432px #fff, 1622px 1334px #fff, 1307px 814px #fff,
    1116px 1589px #fff, 308px 1508px #fff, 1275px 1294px #fff, 433px 1692px #fff,
    807px 648px #fff, 1899px 1820px #fff, 1450px 480px #fff, 1343px 1565px #fff,
    1787px 436px #fff, 335px 991px #fff, 1648px 313px #fff, 373px 1157px #fff,
    856px 1892px #fff, 1266px 991px #fff, 681px 357px #fff, 621px 979px #fff,
    1547px 357px #fff, 1499px 108px #fff, 1968px 1281px #fff, 517px 1684px #fff,
    1829px 436px #fff, 1489px 982px #fff, 419px 143px #fff, 1332px 1365px #fff,
    1670px 1701px #fff, 459px 788px #fff, 170px 790px #fff, 867px 716px #fff,
    774px 1375px #fff, 1400px 1618px #fff, 685px 932px #fff, 1431px 1412px #fff,
    1314px 745px #fff, 356px 1717px #fff, 1508px 716px #fff, 946px 1269px #fff,
    756px 174px #fff, 1856px 446px #fff, 374px 1103px #fff, 1166px 524px #fff,
    520px 1075px #fff, 1611px 363px #fff, 1140px 554px #fff, 1389px 791px #fff,
    1219px 871px #fff, 1885px 897px #fff, 69px 1838px #fff, 785px 332px #fff,
    302px 847px #fff, 206px 933px #fff, 1619px 1302px #fff, 1111px 1805px #fff,
    331px 1033px #fff, 1762px 1935px #fff, 33px 1603px #fff, 143px 625px #fff,
    726px 1072px #fff, 372px 1988px #fff, 1298px 211px #fff, 747px 321px #fff,
    1866px 723px #fff, 709px 715px #fff, 848px 641px #fff, 1198px 61px #fff,
    1847px 668px #fff, 1174px 1457px #fff, 1626px 936px #fff, 1103px 1338px #fff,
    579px 416px #fff, 1383px 1081px #fff, 430px 1305px #fff, 796px 670px #fff,
    1755px 1289px #fff, 799px 231px #fff, 926px 1470px #fff, 158px 42px #fff,
    1808px 687px #fff, 1031px 1318px #fff, 674px 247px #fff, 750px 1951px #fff,
    1040px 184px #fff, 1740px 1450px #fff, 1227px 1915px #fff, 263px 510px #fff,
    1325px 686px #fff, 668px 1077px #fff, 59px 1170px #fff, 1441px 31px #fff,
    1189px 1510px #fff, 457px 544px #fff, 1666px 1155px #fff, 1159px 320px #fff,
    1860px 1443px #fff, 821px 1501px #fff, 998px 541px #fff, 1195px 510px #fff,
    946px 663px #fff, 524px 890px #fff, 132px 257px #fff, 1332px 1674px #fff,
    1089px 1094px #fff, 1590px 1551px #fff, 1428px 1359px #fff,
    1222px 356px #fff, 317px 1351px #fff, 1722px 1497px #fff, 167px 1806px #fff,
    255px 563px #fff, 707px 1399px #fff, 1804px 546px #fff, 276px 227px #fff,
    751px 76px #fff, 1347px 268px #fff, 1684px 357px #fff, 1928px 408px #fff,
    1686px 1891px #fff, 1298px 1550px #fff, 868px 1430px #fff, 1493px 525px #fff,
    814px 463px #fff, 1363px 1879px #fff, 375px 382px #fff, 1143px 197px #fff,
    623px 322px #fff, 1032px 1944px #fff, 1672px 1062px #fff, 614px 1113px #fff,
    1901px 1590px #fff, 407px 872px #fff, 1738px 152px #fff, 54px 1399px #fff,
    1778px 1657px #fff, 968px 717px #fff, 423px 1966px #fff, 1883px 1099px #fff,
    1746px 274px #fff, 9px 1458px #fff, 850px 1692px #fff, 1075px 1562px #fff,
    1724px 873px #fff, 1769px 1479px #fff, 1765px 492px #fff, 1998px 1479px #fff,
    1016px 1216px #fff, 811px 356px #fff, 474px 401px #fff, 226px 705px #fff,
    1080px 591px #fff, 532px 1003px #fff, 1601px 1287px #fff, 306px 1207px #fff,
    736px 1081px #fff, 458px 780px #fff, 1431px 369px #fff, 214px 869px #fff,
    959px 1182px #fff, 467px 1928px #fff, 1416px 1777px #fff, 1477px 1759px #fff,
    476px 840px #fff, 1309px 1441px #fff, 1156px 1128px #fff, 1080px 1905px #fff,
    1651px 1715px #fff, 96px 257px #fff, 115px 586px #fff, 1160px 1651px #fff,
    210px 1471px #fff, 807px 36px #fff, 1841px 188px #fff, 682px 1119px #fff,
    522px 465px #fff, 1430px 1009px #fff;
  animation: ${animationStar} 100s linear infinite;

  &:after {
    content: " ";
    position: absolute;
    top: 2000px;
    width: 2px;
    height: 2px;
    background: transparent;
    box-shadow: 368px 598px #fff, 1373px 195px #fff, 279px 85px #fff,
      115px 1514px #fff, 1139px 783px #fff, 797px 816px #fff, 572px 88px #fff,
      490px 789px #fff, 290px 46px #fff, 606px 616px #fff, 1417px 402px #fff,
      1572px 737px #fff, 466px 1146px #fff, 421px 621px #fff, 1863px 1096px #fff,
      423px 1615px #fff, 1915px 790px #fff, 862px 847px #fff, 303px 110px #fff,
      1050px 1531px #fff, 1786px 1072px #fff, 795px 1665px #fff,
      1727px 29px #fff, 1892px 440px #fff, 457px 1432px #fff, 1622px 1334px #fff,
      1307px 814px #fff, 1116px 1589px #fff, 308px 1508px #fff,
      1275px 1294px #fff, 433px 1692px #fff, 807px 648px #fff,
      1899px 1820px #fff, 1450px 480px #fff, 1343px 1565px #fff,
      1787px 436px #fff, 335px 991px #fff, 1648px 313px #fff, 373px 1157px #fff,
      856px 1892px #fff, 1266px 991px #fff, 681px 357px #fff, 621px 979px #fff,
      1547px 357px #fff, 1499px 108px #fff, 1968px 1281px #fff,
      517px 1684px #fff, 1829px 436px #fff, 1489px 982px #fff, 419px 143px #fff,
      1332px 1365px #fff, 1670px 1701px #fff, 459px 788px #fff, 170px 790px #fff,
      867px 716px #fff, 774px 1375px #fff, 1400px 1618px #fff, 685px 932px #fff,
      1431px 1412px #fff, 1314px 745px #fff, 356px 1717px #fff,
      1508px 716px #fff, 946px 1269px #fff, 756px 174px #fff, 1856px 446px #fff,
      374px 1103px #fff, 1166px 524px #fff, 520px 1075px #fff, 1611px 363px #fff,
      1140px 554px #fff, 1389px 791px #fff, 1219px 871px #fff, 1885px 897px #fff,
      69px 1838px #fff, 785px 332px #fff, 302px 847px #fff, 206px 933px #fff,
      1619px 1302px #fff, 1111px 1805px #fff, 331px 1033px #fff,
      1762px 1935px #fff, 33px 1603px #fff, 143px 625px #fff, 726px 1072px #fff,
      372px 1988px #fff, 1298px 211px #fff, 747px 321px #fff, 1866px 723px #fff,
      709px 715px #fff, 848px 641px #fff, 1198px 61px #fff, 1847px 668px #fff,
      1174px 1457px #fff, 1626px 936px #fff, 1103px 1338px #fff,
      579px 416px #fff, 1383px 1081px #fff, 430px 1305px #fff, 796px 670px #fff,
      1755px 1289px #fff, 799px 231px #fff, 926px 1470px #fff, 158px 42px #fff,
      1808px 687px #fff, 1031px 1318px #fff, 674px 247px #fff, 750px 1951px #fff,
      1040px 184px #fff, 1740px 1450px #fff, 1227px 1915px #fff,
      263px 510px #fff, 1325px 686px #fff, 668px 1077px #fff, 59px 1170px #fff,
      1441px 31px #fff, 1189px 1510px #fff, 457px 544px #fff, 1666px 1155px #fff,
      1159px 320px #fff, 1860px 1443px #fff, 821px 1501px #fff, 998px 541px #fff,
      1195px 510px #fff, 946px 663px #fff, 524px 890px #fff, 132px 257px #fff,
      1332px 1674px #fff, 1089px 1094px #fff, 1590px 1551px #fff,
      1428px 1359px #fff, 1222px 356px #fff, 317px 1351px #fff,
      1722px 1497px #fff, 167px 1806px #fff, 255px 563px #fff, 707px 1399px #fff,
      1804px 546px #fff, 276px 227px #fff, 751px 76px #fff, 1347px 268px #fff,
      1684px 357px #fff, 1928px 408px #fff, 1686px 1891px #fff,
      1298px 1550px #fff, 868px 1430px #fff, 1493px 525px #fff, 814px 463px #fff,
      1363px 1879px #fff, 375px 382px #fff, 1143px 197px #fff, 623px 322px #fff,
      1032px 1944px #fff, 1672px 1062px #fff, 614px 1113px #fff,
      1901px 1590px #fff, 407px 872px #fff, 1738px 152px #fff, 54px 1399px #fff,
      1778px 1657px #fff, 968px 717px #fff, 423px 1966px #fff,
      1883px 1099px #fff, 1746px 274px #fff, 9px 1458px #fff, 850px 1692px #fff,
      1075px 1562px #fff, 1724px 873px #fff, 1769px 1479px #fff,
      1765px 492px #fff, 1998px 1479px #fff, 1016px 1216px #fff,
      811px 356px #fff, 474px 401px #fff, 226px 705px #fff, 1080px 591px #fff,
      532px 1003px #fff, 1601px 1287px #fff, 306px 1207px #fff,
      736px 1081px #fff, 458px 780px #fff, 1431px 369px #fff, 214px 869px #fff,
      959px 1182px #fff, 467px 1928px #fff, 1416px 1777px #fff,
      1477px 1759px #fff, 476px 840px #fff, 1309px 1441px #fff,
      1156px 1128px #fff, 1080px 1905px #fff, 1651px 1715px #fff,
      96px 257px #fff, 115px 586px #fff, 1160px 1651px #fff, 210px 1471px #fff,
      807px 36px #fff, 1841px 188px #fff, 682px 1119px #fff, 522px 465px #fff,
      1430px 1009px #fff;
  }
`;

const LargeStars = styled.div`
  width: 3px;
  height: 3px;
  background: transparent;
  box-shadow: 225px 1037px #fff, 1447px 1587px #fff, 1286px 203px #fff,
    824px 524px #fff, 1613px 209px #fff, 381px 115px #fff, 1144px 1083px #fff,
    752px 1464px #fff, 908px 1861px #fff, 465px 1328px #fff, 439px 1848px #fff,
    1136px 835px #fff, 1364px 1013px #fff, 155px 1911px #fff, 1819px 521px #fff,
    1040px 706px #fff, 996px 1633px #fff, 618px 37px #fff, 1339px 360px #fff,
    1722px 135px #fff, 777px 1283px #fff, 1034px 1201px #fff, 1571px 367px #fff,
    1801px 813px #fff, 721px 870px #fff, 1090px 1605px #fff, 1553px 1373px #fff,
    1532px 718px #fff, 1672px 7px #fff, 304px 196px #fff, 713px 1754px #fff,
    1410px 1404px #fff, 1054px 1400px #fff, 1006px 872px #fff, 792px 1562px #fff,
    128px 400px #fff, 686px 1658px #fff, 1451px 147px #fff, 1416px 1448px #fff,
    918px 1834px #fff, 1182px 343px #fff, 722px 587px #fff, 1697px 1768px #fff,
    287px 38px #fff, 974px 1133px #fff, 1119px 1737px #fff, 546px 1110px #fff,
    1283px 1233px #fff, 735px 191px #fff, 826px 566px #fff, 1449px 1769px #fff,
    750px 980px #fff, 818px 60px #fff, 1824px 944px #fff, 1140px 1750px #fff,
    1811px 438px #fff, 736px 983px #fff, 1779px 1892px #fff, 805px 425px #fff,
    389px 130px #fff, 1755px 919px #fff, 1018px 1885px #fff, 803px 166px #fff,
    1071px 1160px #fff, 1758px 1926px #fff, 1882px 1508px #fff, 367px 54px #fff,
    1013px 210px #fff, 143px 787px #fff, 958px 746px #fff, 1599px 1877px #fff,
    296px 349px #fff, 1278px 1263px #fff, 211px 1080px #fff, 1160px 1469px #fff,
    1119px 12px #fff, 192px 1205px #fff, 1664px 916px #fff, 338px 746px #fff,
    847px 1862px #fff, 995px 753px #fff, 628px 1752px #fff, 1650px 1821px #fff,
    1947px 3px #fff, 353px 7px #fff, 503px 825px #fff, 245px 1821px #fff,
    501px 1874px #fff, 76px 1579px #fff, 694px 1089px #fff, 243px 117px #fff,
    869px 1828px #fff, 745px 1820px #fff, 1057px 1740px #fff, 1825px 1395px #fff,
    1045px 1485px #fff, 648px 1935px #fff, 488px 1510px #fff, 1766px 1883px #fff,
    1879px 410px #fff;
  animation: ${animationStar} 150s linear infinite;

  &:after {
    content: " ";
    position: absolute;
    top: 2000px;
    width: 3px;
    height: 3px;
    background: transparent;
    box-shadow: 225px 1037px #fff, 1447px 1587px #fff, 1286px 203px #fff,
      824px 524px #fff, 1613px 209px #fff, 381px 115px #fff, 1144px 1083px #fff,
      752px 1464px #fff, 908px 1861px #fff, 465px 1328px #fff, 439px 1848px #fff,
      1136px 835px #fff, 1364px 1013px #fff, 155px 1911px #fff,
      1819px 521px #fff, 1040px 706px #fff, 996px 1633px #fff, 618px 37px #fff,
      1339px 360px #fff, 1722px 135px #fff, 777px 1283px #fff,
      1034px 1201px #fff, 1571px 367px #fff, 1801px 813px #fff, 721px 870px #fff,
      1090px 1605px #fff, 1553px 1373px #fff, 1532px 718px #fff, 1672px 7px #fff,
      304px 196px #fff, 713px 1754px #fff, 1410px 1404px #fff,
      1054px 1400px #fff, 1006px 872px #fff, 792px 1562px #fff, 128px 400px #fff,
      686px 1658px #fff, 1451px 147px #fff, 1416px 1448px #fff,
      918px 1834px #fff, 1182px 343px #fff, 722px 587px #fff, 1697px 1768px #fff,
      287px 38px #fff, 974px 1133px #fff, 1119px 1737px #fff, 546px 1110px #fff,
      1283px 1233px #fff, 735px 191px #fff, 826px 566px #fff, 1449px 1769px #fff,
      750px 980px #fff, 818px 60px #fff, 1824px 944px #fff, 1140px 1750px #fff,
      1811px 438px #fff, 736px 983px #fff, 1779px 1892px #fff, 805px 425px #fff,
      389px 130px #fff, 1755px 919px #fff, 1018px 1885px #fff, 803px 166px #fff,
      1071px 1160px #fff, 1758px 1926px #fff, 1882px 1508px #fff,
      367px 54px #fff, 1013px 210px #fff, 143px 787px #fff, 958px 746px #fff,
      1599px 1877px #fff, 296px 349px #fff, 1278px 1263px #fff,
      211px 1080px #fff, 1160px 1469px #fff, 1119px 12px #fff, 192px 1205px #fff,
      1664px 916px #fff, 338px 746px #fff, 847px 1862px #fff, 995px 753px #fff,
      628px 1752px #fff, 1650px 1821px #fff, 1947px 3px #fff, 353px 7px #fff,
      503px 825px #fff, 245px 1821px #fff, 501px 1874px #fff, 76px 1579px #fff,
      694px 1089px #fff, 243px 117px #fff, 869px 1828px #fff, 745px 1820px #fff,
      1057px 1740px #fff, 1825px 1395px #fff, 1045px 1485px #fff,
      648px 1935px #fff, 488px 1510px #fff, 1766px 1883px #fff,
      1879px 410px #fff;
  }
`;

const WrapperContent = styled.div`
  position: absolute;
  top: 0;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
`;

const Empty = styled.div``;

const WrapperText = styled.div`
  @media (min-width: 768px) {
    margin: 0em 4em;
  }
  @media (min-width: 512px) {
    margin: 0em 2em;
  }

  margin: 0em 1em;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: end;

  ${font}
`;

const Logo = styled.img`
  height: 100px;
`;

const PrimaryText = styled.div`
  @media (min-width: 512px) {
    font-size: 24px;
    line-height: 32px;
  }

  font-size: 18px;
  line-height: 24px;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
`;
const SecondaryText = styled.p`
  @media (min-width: 512px) {
    font-size: 24px;
    line-height: 32px;
  }

  font-size: 18px;
  line-height: 24px;

  font-family: "Kodchasan", sans-serif;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  text-transform: uppercase;
`;

const WrapperSocial = styled.div`
  display: flex;
  margin-bottom: 2em;
  width: 180px;
  align-items: center;
  justify-content: space-around;
`;

const SocialText = styled.p`
  font-family: "Kodchasan", sans-serif;
  font-size: 12px;
  font-weight: 700;
  line-height: 16px;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;
  margin: 0;
`;

const SocialLink = styled.a`
  cursor: pointer;
  text-align: none;
  height: 16px;
  margin: 0;
  padding: 0;
`;

const SocialIcon = styled.img`
  height: 100%;
  vertical-align: unset;
`;

return (
  <Wrapper>
    <WrapperStars>
      <SmallStars />
      <MediumStars />
      <LargeStars />
    </WrapperStars>
    <WrapperContent>
      <Empty />
      <WrapperText>
        <Logo
          src="https://ipfs.near.social/ipfs/bafkreidsc7fcwi3urcpew2fiuvqw47d7i5bxydd64ttfqqp4f2m577khha"
          alt="Near Box logo"
        />
        <PrimaryText>
          Our gifts are being crafted under the watchful eyes of
          extraterrestrials to bring you something truly unique from space!
        </PrimaryText>
        <SecondaryText>Come again later</SecondaryText>
      </WrapperText>
      <WrapperSocial>
        <SocialText>Follow us</SocialText>
        <SocialLink href="https://twitter.com/nearuaguild" target="_blank">
          <SocialIcon
            src="https://ipfs.near.social/ipfs/bafkreibhvlipldq5qnolfb74ufbgqkbcwlim5vvtk3mbz6ujvbsar6fesq"
            alt="Twitter"
          />
        </SocialLink>
        <SocialLink href="https://t.me/nearprotocolua" target="_blank">
          <SocialIcon
            src="https://ipfs.near.social/ipfs/bafkreihcqu65spu6o5z6vw5atbjx7iqphzvlss3hvz4l7bj3syhvavzf5a"
            alt="Telegram"
          />
        </SocialLink>
        <SocialLink
          href="https://near.org/near/widget/ProfilePage?accountId=nearukraineguild.near"
          target="_blank"
        >
          <SocialIcon
            src="https://ipfs.near.social/ipfs/bafkreier4aong3uumu4ndl6iahol2kgeisfqtl6c237x3q34ql6smjvare"
            alt="Near Social"
          />
        </SocialLink>
      </WrapperSocial>
    </WrapperContent>
  </Wrapper>
);
