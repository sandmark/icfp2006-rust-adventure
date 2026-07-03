# シカゴシティ

## Overview

- **洞察**: アイテム名、特徴は静的であるようだ
- **事実**: アイテムが多いほど `go`, `look` に時間がかかる (数秒〜十数秒)

## 地図

| <None>             | 52 St.   & Dorchester | 52 St.   & Blackstone | 52 St.   & Harper |
| <None>             | 53 St.   & Dorchester | 53 St.   & Blackstone | 53 St.   & Harper |
| 54 St. & Ridgewood | 54 St.   & Dorchester | 54 St.   & Blackstone | 54 St.   & Harper |
| <None>             | 54 Place & Dorchester | 54 Place & Blackstone | 54 Place & Harper |

**54 Street and Ridgewood** からスタートする。

X = (Ridgewood, Dorchester, Blackstone, Harper)
Y = (52-St, 53-St, 54-St, 54-Place)

`downloader`, `uploader` の機能によってはさらに拡張するのだろうか？

## Tactics

チュートリアル (ジャンクルーム) では、依存関係が一部屋で完結していた。
シカゴシティではいくつものエリアを渡り歩きながら、
作れるものから作っていくことになりそうだ。

`downloader`, `uploader` を作るためには、それぞれ 5 つの材料が必要だ。
その材料にもそれぞれ材料が必要。
となると、「何から作るべきか」は自ずと定まるものと考えられる
**(`drop` が noop かどうかは要調査)。**

### Session + State vs. Boot Script

- **可変状態**

`Session` が `GameState` を持つこともできる。
インスタンスの地形情報、アイテム、依存関係を持つ。
これは **データが動的ならリターンがある** が、静的であるなら過剰であり、
**可変であることそのものが脆弱**というリスクがある。
発展させるならイマーシブな `make` コマンドなどが実装できる。

- **スクリプト**

**地形や依存関係が常に一定なら、実行時に経路探索する必要がそもそもない。**
`Session` が `Logger` を持ち、引数で xml ファイルを読み込むことで
UM を起動することなく探索が可能だ。
**ただしデータが動的である場合や未知の展開には脆い。**

## Whole Logs

- input:
```
    n
    take bolt
    take spring
    take button
    take processor
    take pill
    inc pill
    take radio
    take cache
    combine processor with cache
    take blue transistor
    combine radio with transistor
    take antenna
    inc antenna
    inc spring
    take screw
    take motherboard
    combine motherboard with screw
    take a-1920-ixb
    combine a-1920-ixb with radio
    combine a-1920-ixb with bolt
    combine a-1920-ixb with processor
    take transistor
    take keypad
    combine a-1920-ixb with transistor
    combine motherboard with a-1920-ixb
    combine keypad with motherboard
    combine keypad with button
    s
    use keypad
    east
    north
    north
    east
    east
    south
    west
    south
    east
    south
    west
    west
```

- output:

```
12:00:00 1/1/19100
Welcome to Universal Machine IX (UMIX).

This machine is a shared resource. Please do not log
in to multiple simultaneous UMIX servers. No game playing
is allowed.

Please log in (use 'guest' for visitor access).
;login: password: logged in as howie


% [Building vocabulary]
[Initializing command processor]
[Populating environment]
Room With a Door

You are in a room with a mechanical door. You will probably need
to use a keypad to unlock it. A hallway leads north.
There is a pamphlet here.
Underneath the pamphlet, there is a manifesto.

>: `switch` mode: XML
--- ジャンクルーム ---
がらくたが山積みの部屋だ。廊下が南に続いている。

bolt: "quite useful for securing all sorts of things"
  特徴: なし
  状態: 無傷
spring: "tightly coiled"
  特徴: なし
  状態: 無傷
button: "labeled 6"
  特徴: なし
  状態: 無傷
processor: "from the elusive 19x86 line"
  特徴: なし
  状態: 壊れている: cache が足りない
pill: "tempting looking"
  特徴: red
  状態: 無傷
radio: "a hi-fi AM/FM stereophonic radio"
  特徴: なし
  状態: 壊れている: transistor, antenna が足りない
cache: "fully-associative"
  特徴: なし
  状態: 無傷
transistor: "PNP-complete"
  特徴: blue
  状態: 無傷
antenna: "appropriate for receiving transmissions between 30 kHz and 30 MHz"
  特徴: なし
  状態: 無傷
screw: "not from a Dutch company"
  特徴: なし
  状態: 無傷
motherboard: "well-used"
  特徴: なし
  状態: 壊れている: A-1920-IXB, screw が足りない
A-1920-IXB: "an exemplary instance of part number A-1920-IXB"
  特徴: なし
  状態: 壊れている: antenna を欠いた radio, processor, bolt が足りず、それらを補ってもなお transistor を欠く
transistor: "NPN-complete"
  特徴: red
  状態: 無傷
keypad: "labeled "use me""
  特徴: なし
  状態: 壊れている: motherboard, button が足りない
trash: "of absolutely no value"
  特徴: なし
  状態: 無傷

bolt を拾った

spring を拾った

button を拾った

processor を拾った

pill を拾った

ADVTR.INC=5@999999|f95731ab88952dfa4cb326fb99c085f
pill を火葬した…

radio を拾った

cache を拾った

ADVTR.CMB=5@999999|764e8a851411c66106e130374d8abbb
processor と cache を組み合わせた

transistor を拾った

radio と transistor を組み合わせた

antenna を拾った

antenna を火葬した…

spring を火葬した…

screw を拾った

motherboard を拾った

motherboard と screw を組み合わせた

A-1920-IXB を拾った

A-1920-IXB と radio を組み合わせた

A-1920-IXB と bolt を組み合わせた

A-1920-IXB と processor を組み合わせた

transistor を拾った

keypad を拾った

A-1920-IXB と transistor を組み合わせた

motherboard と A-1920-IXB を組み合わせた

keypad と motherboard を組み合わせた

keypad と button を組み合わせた

--- ドアのある部屋 ---
機械式のドアがある部屋にいる。開けるには keypad を使う必要がありそうだ。廊下が北に続いている。

pamphlet: "ありがちな市のチラシにはこう書かれている。『シカゴ市の廃棄物再利用計画』。近代的ゴミ区分とサイボーグ労働力を組み合わせることで、無駄を最小限に、かつ出費を抑えながら美しい街作りを実現させています。本プログラムのモットー「ある人のゴミは他の誰かの宝物」に基づき、回収された不用品は修理された上で、新品で買う予定だった他の住民のもとへ再頒布されます。住民の皆様は回収日に、いらないものを歩道に山積みにして市のプログラムに貢献しましょう。"
  特徴: なし
  状態: 無傷
manifesto: "[検閲]"
  特徴: なし
  状態: 無傷

ADVTR.KEY=20@999999|36995486a5be3bd747d778916846d2d
keypad を使った
---
鍵を開けてドアを開いた。通り抜けると、シカゴの大通りにいるらしいことがわかった。もはや戻る必要はないだろう。背後のドアが閉まる音を背中で聞いた。
--- 交差点: 54番街／ドーチェスター・アベニュー ---
54番街とドーチェスター・アベニューの交差点に立っている。ここから北、東、南、西へ進むことができる。

X-9247-GWE: "an exemplary instance of part number X-9247-GWE"
  特徴: orange-red
  状態: 無傷
V-0010-XBD: "an exemplary instance of part number V-0010-XBD"
  特徴: magenta
  状態: 壊れている: X-9247-GWE が足りない
F-1403-QDS: "an exemplary instance of part number F-1403-QDS"
  特徴: pumpkin
  状態: 無傷
P-5065-WQO: "an exemplary instance of part number P-5065-WQO"
  特徴: heavy
  状態: 壊れている: F-1403-QDS が足りず、それを補ってもなお B-4832-LAL を欠き、さらに T-6678-BTV を欠く
B-4832-LAL: "an exemplary instance of part number B-4832-LAL"
  特徴: taupe
  状態: 無傷
L-6458-RNH: "an exemplary instance of part number L-6458-RNH"
  特徴: gray40
  状態: 壊れている: T-6678-BTV を欠いた P-5065-WQO が足りない
T-9887-OFC: "an exemplary instance of part number T-9887-OFC"
  特徴: eggplant
  状態: 壊れている: H-9887-MKY が足りず、それを補ってもなお X-6458-TIJ を欠く
Z-1623-CEK: "an exemplary instance of part number Z-1623-CEK"
  特徴: indigo
  状態: 壊れている: L-6458-RNH が足りず、それを補ってもなお D-4292-HHR を欠く
H-9887-MKY: "an exemplary instance of part number H-9887-MKY"
  特徴: yellow-green
  状態: 無傷
F-6678-DOX: "an exemplary instance of part number F-6678-DOX"
  特徴: shiny
  状態: 壊れている: V-0010-XBD が足りず、それを補ってもなお J-9247-IRG を欠く
R-1403-SXU: "an exemplary instance of part number R-1403-SXU"
  特徴: pale-green
  状態: 無傷
USB cable: "あらゆる高速 Universal Sand Bus 2.0 デバイスと互換性あり"
  特徴: なし
  状態: 壊れている: N-4832-NUN が足りず、それを補ってもなお F-6678-DOX を欠き、さらに X-6458-TIJ を欠いた T-9887-OFC を欠く
N-4832-NUN: "an exemplary instance of part number N-4832-NUN"
  特徴: sienna
  状態: 無傷
J-9247-IRG: "an exemplary instance of part number J-9247-IRG"
  特徴: slate-gray
  状態: 無傷
B-5065-YLQ: "an exemplary instance of part number B-5065-YLQ"
  特徴: dim-gray
  状態: 無傷

--- 交差点: 53番街／ドーチェスター・アベニュー ---
53番街とドーチェスター・アベニューの交差点に立っている。ここから北、東、南へ進むことができる。

N-1623-AOE: "an exemplary instance of part number N-1623-AOE"
  特徴: fern-green
  状態: 無傷
R-4292-FRL: "an exemplary instance of part number R-4292-FRL"
  特徴: burgundy
  状態: 壊れている: D-5065-UBI を欠いた Z-6458-PXZ が足りず、それを補ってもなお V-9887-KUS を欠く
F-6458-DDN: "an exemplary instance of part number F-6458-DDN"
  特徴: pale-magenta
  状態: 壊れている: J-5065-IGU が足りない
H-1623-MYO: "an exemplary instance of part number H-1623-MYO"
  特徴: peach-yellow
  状態: 壊れている: F-9247-QRI を欠いた T-6458-BIL, P-9887-WFE が足りず、それらを補ってもなお L-4292-RCV を欠く
H-4292-ZHF: "an exemplary instance of part number H-4292-ZHF"
  特徴: rotating
  状態: 壊れている: J-4832-VUP が足りない
R-6458-FXP: "an exemplary instance of part number R-6458-FXP"
  特徴: low-carb
  状態: 壊れている: V-5065-KBW, P-9887-WFE を欠いた H-1623-MYO, J-4832-VUP を欠いた H-4292-ZHF が足りない
T-6458-BIL: "an exemplary instance of part number T-6458-BIL"
  特徴: mysterious
  状態: 壊れている: F-9247-QRI が足りず、それを補ってもなお B-6678-LOZ を欠いた T-5065-OQC を欠き、さらに X-5065-GLS を欠く
R-9247-SMK: "an exemplary instance of part number R-9247-SMK"
  特徴: brass
  状態: 壊れている: V-4832-XPR が足りない
Z-1403-CSY: "an exemplary instance of part number Z-1403-CSY"
  特徴: puce
  状態: 壊れている: D-0010-HVH が足りない
N-6678-NJD: "an exemplary instance of part number N-6678-NJD"
  特徴: pink
  状態: 壊れている: V-4832-XPR を欠いた R-9247-SMK が足りない
X-4292-TWX: "an exemplary instance of part number X-4292-TWX"
  特徴: jade
  状態: 壊れている: N-6678-NJD, B-9887-YAG が足りない
Z-6678-PEF: "an exemplary instance of part number Z-6678-PEF"
  特徴: flax
  状態: 壊れている: B-9887-YAG を欠いた X-4292-TWX が足りない
H-4832-ZKT: "an exemplary instance of part number H-4832-ZKT"
  特徴: pale-blue
  状態: 壊れている: L-1403-ENC が足りない
P-0010-JQJ: "an exemplary instance of part number P-0010-JQJ"
  特徴: gray60
  状態: 壊れている: T-1623-OTQ が足りず、それを補ってもなお P-9247-WCO を欠く
J-1403-IDG: "an exemplary instance of part number J-1403-IDG"
  特徴: olive-green
  状態: 壊れている: L-1403-ENC を欠いた H-4832-ZKT が足りない
D-9247-UHM: "an exemplary instance of part number D-9247-UHM"
  特徴: swamp-green
  状態: 無傷
N-6458-NDX: "an exemplary instance of part number N-6458-NDX"
  特徴: khaki
  状態: 壊れている: Z-6678-PEF, J-1403-IDG, P-9247-WCO が足りず、それらを補ってもなお L-6678-RYH を欠く
V-9887-KUS: "an exemplary instance of part number V-9887-KUS"
  特徴: red-violet
  状態: 壊れている: V-5065-KBW を欠いた R-6458-FXP, T-4832-BFV, V-9887-KUS を欠いた H-6678-ZEP が足りない
N-0010-NGN: "an exemplary instance of part number N-0010-NGN"
  特徴: tea-green
  状態: 壊れている: P-9247-WCO を欠いた N-6458-NDX, R-1623-SJU が足りない
X-1403-GIE: "an exemplary instance of part number X-1403-GIE"
  特徴: cinnamon
  状態: 壊れている: N-9887-AUI を欠いた F-1623-QOS が足りず、それを補ってもなお B-0010-LLL を欠く
T-4832-BFV: "an exemplary instance of part number T-4832-BFV"
  特徴: gray20
  状態: 無傷
D-6458-HSR: "an exemplary instance of part number D-6458-HSR"
  特徴: beige
  状態: 壊れている: H-5065-MVY が足りない
F-4832-DAX: "an exemplary instance of part number F-4832-DAX"
  特徴: ghost-white
  状態: 壊れている: J-1403-IDG, H-5065-MVY を欠いた D-6458-HSR, Z-9887-CPK を欠いた V-4292-XMD, R-1623-SJU を欠いた N-0010-NGN が足りない
V-4292-XMD: "an exemplary instance of part number V-4292-XMD"
  特徴: olive-green
  状態: 壊れている: Z-9887-CPK が足りない
H-4292-ZHF: "an exemplary instance of part number H-4292-ZHF"
  特徴: light-brown
  状態: 壊れている: L-9887-EKM が足りない
X-6678-TTJ: "an exemplary instance of part number X-6678-TTJ"
  特徴: lawn-green
  状態: 壊れている: X-9887-GFO, J-1403-IDG を欠いた F-4832-DAX が足りず、それらを補ってもなお B-9247-YWQ を欠く
T-4292-BCH: "an exemplary instance of part number T-4292-BCH"
  特徴: aquamarine
  状態: 壊れている: X-9887-GFO, X-9887-GFO を欠いた X-6678-TTJ が足りない
P-6458-JNT: "an exemplary instance of part number P-6458-JNT"
  特徴: maroon
  状態: 壊れている: T-5065-OQC が足りない
Z-0010-PBP: "an exemplary instance of part number Z-0010-PBP"
  特徴: rust
  状態: 壊れている: D-1623-UEW, L-9887-EKM を欠いた H-4292-ZHF が足りない
F-5065-QLE: "an exemplary instance of part number F-5065-QLE"
  特徴: cyan
  状態: 壊れている: D-1623-UEW を欠いた Z-0010-PBP, V-1403-KXI を欠いた R-4832-FUZ が足りず、それらを補ってもなお N-9247-ARS を欠き、さらに J-6678-VOL を欠く
V-9887-KUS: "an exemplary instance of part number V-9887-KUS"
  特徴: lavender-blush
  状態: 壊れている: H-1403-MSK が足りない
B-6458-LIV: "an exemplary instance of part number B-6458-LIV"
  特徴: olive-drab
  状態: 無傷
D-5065-UBI: "an exemplary instance of part number D-5065-UBI"
  特徴: plum
  状態: 壊れている: Z-9247-CMU, X-9887-GFO を欠いた T-4292-BCH が足りず、それらを補ってもなお V-6678-XJN を欠き、さらに R-5065-SGG を欠く
L-9247-EHW: "an exemplary instance of part number L-9247-EHW"
  特徴: magenta
  状態: 壊れている: T-1403-ONM を欠いた P-4832-JKF, Z-9247-CMU を欠いた D-5065-UBI, P-1623-WYY が足りない
P-1623-WYY: "an exemplary instance of part number P-1623-WYY"
  特徴: ochre
  状態: 無傷
D-4832-HPD: "an exemplary instance of part number D-4832-HPD"
  特徴: gray60
  状態: 壊れている: L-0010-RVR を欠いた H-1403-MSK が足りない
B-1623-YTC: "an exemplary instance of part number B-1623-YTC"
  特徴: chestnut
  状態: 壊れている: F-4292-DWJ, J-9887-IAQ を欠いた Z-6458-PXZ が足りない
Z-6458-PXZ: "an exemplary instance of part number Z-6458-PXZ"
  特徴: rust
  状態: 壊れている: N-6458-NDX が足りない
Z-6458-PXZ: "an exemplary instance of part number Z-6458-PXZ"
  特徴: robin-egg-blue
  状態: 壊れている: J-9887-IAQ が足りない
display: "文字データを表示できる携帯型デバイス"
  特徴: なし
  状態: 壊れている: T-1403-ONM を欠いた P-4832-JKF を欠いた L-9247-EHW, F-4292-DWJ を欠いた B-1623-YTC, D-5065-UBI を欠いた Z-6458-PXZ を欠いた R-4292-FRL, N-1623-AOE が足りない
X-0010-TQT: "an exemplary instance of part number X-0010-TQT"
  特徴: navajo-white
  状態: 壊れている: H-6678-ZEP が足りない

--- 52nd Street and Dorchester Avenue ---
You are standing at the corner of 52nd Street and Dorchester Avenue. From here, you can go east or south.

V-5065-KLY: "an exemplary instance of part number V-5065-KLY"
  特徴: cerise
  状態: 壊れている: P-0010-JBL を欠いた L-1403-EXE を欠いた H-4832-ZUV が足りず、それを補ってもなお Z-6678-POH を欠く
J-5065-IQW: "an exemplary instance of part number J-5065-IQW"
  特徴: foreign
  状態: 無傷
X-5065-GVU: "an exemplary instance of part number X-5065-GVU"
  特徴: floating
  状態: 無傷
B-6678-LYD: "an exemplary instance of part number B-6678-LYD"
  特徴: chartreuse
  状態: 無傷
N-1403-AIY: "an exemplary instance of part number N-1403-AIY"
  特徴: lavender
  状態: 無傷
R-0010-FLH: "an exemplary instance of part number R-0010-FLH"
  特徴: bondi-blue
  状態: 無傷
V-1623-KOO: "an exemplary instance of part number V-1623-KOO"
  特徴: old
  状態: 無傷
Z-1403-CDC: "an exemplary instance of part number Z-1403-CDC"
  特徴: deep-sky-blue
  状態: 壊れている: B-6678-LYD が足りず、それを補ってもなお H-1623-MJQ を欠いた D-0010-HGJ を欠く
B-9887-YKI: "an exemplary instance of part number B-9887-YKI"
  特徴: hot-pink
  状態: 無傷
P-9887-WPG: "an exemplary instance of part number P-9887-WPG"
  特徴: ultramarine
  状態: 壊れている: T-6458-BSN が足りない
F-9247-QCK: "an exemplary instance of part number F-9247-QCK"
  特徴: fern-green
  状態: 壊れている: J-4832-VFR が足りない
B-9247-YHS: "an exemplary instance of part number B-9247-YHS"
  特徴: lime-green
  状態: 壊れている: F-4832-DKZ が足りない
J-1403-INI: "an exemplary instance of part number J-1403-INI"
  特徴: reverse-chirality
  状態: 壊れている: X-4292-THZ が足りず、それを補ってもなお N-0010-NQP を欠く
L-4292-RMX: "an exemplary instance of part number L-4292-RMX"
  特徴: school-bus-yellow
  状態: 無傷
R-6458-FIR: "an exemplary instance of part number R-6458-FIR"
  特徴: yellow-green
  状態: 壊れている: L-4292-RMX が足りず、それを補ってもなお T-1623-OES を欠く
R-9247-SWM: "an exemplary instance of part number R-9247-SWM"
  特徴: puce
  状態: 壊れている: V-4832-XAT が足りない
V-4832-XAT: "an exemplary instance of part number V-4832-XAT"
  特徴: carrot
  状態: 無傷
H-4832-ZUV: "an exemplary instance of part number H-4832-ZUV"
  特徴: discounted
  状態: 壊れている: P-0010-JBL を欠いた L-1403-EXE が足りない
N-6678-NTF: "an exemplary instance of part number N-6678-NTF"
  特徴: hot-pink
  状態: 無傷
L-6678-RJJ: "an exemplary instance of part number L-6678-RJJ"
  特徴: exceptional
  状態: 壊れている: N-9887-AFK を欠いた F-1623-QYU を欠いた B-0010-LVN が足りず、それを補ってもなお T-4832-BPX を欠いた P-9247-WMQ を欠く
F-6458-DNP: "an exemplary instance of part number F-6458-DNP"
  特徴: purple
  状態: 無傷
X-4292-THZ: "an exemplary instance of part number X-4292-THZ"
  特徴: lime
  状態: 無傷
L-9887-EUO: "an exemplary instance of part number L-9887-EUO"
  特徴: beige
  状態: 壊れている: X-1403-GSG が足りず、それを補ってもなお H-5065-MGC を欠き、さらに F-4832-DKZ を欠いた B-9247-YHS を欠く
T-1623-OES: "an exemplary instance of part number T-1623-OES"
  特徴: deep-pink
  状態: 無傷
X-1403-GSG: "an exemplary instance of part number X-1403-GSG"
  特徴: plum
  状態: 壊れている: D-9247-URO が足りない
D-9247-URO: "an exemplary instance of part number D-9247-URO"
  特徴: malachite
  状態: 無傷
N-0010-NQP: "an exemplary instance of part number N-0010-NQP"
  特徴: fuchsia
  状態: 壊れている: R-6458-FIR が足りず、それを補ってもなお D-6458-HDT を欠いた Z-9887-CAM を欠いた V-4292-XWF を欠く
B-0010-LVN: "an exemplary instance of part number B-0010-LVN"
  特徴: dodger-blue
  状態: 壊れている: N-9887-AFK を欠いた F-1623-QYU が足りない
D-1623-UOY: "an exemplary instance of part number D-1623-UOY"
  特徴: pine-green
  状態: 壊れている: H-4292-ZRH が足りない
V-4292-XWF: "an exemplary instance of part number V-4292-XWF"
  特徴: floral-white
  状態: 壊れている: T-4832-BPX を欠いた P-9247-WMQ を欠いた L-6678-RJJ が足りず、それを補ってもなお D-6458-HDT を欠いた Z-9887-CAM を欠く
H-5065-MGC: "an exemplary instance of part number H-5065-MGC"
  特徴: bisque
  状態: 無傷
V-1403-KIK: "an exemplary instance of part number V-1403-KIK"
  特徴: violet
  状態: 壊れている: T-5065-OBE を欠いた P-6458-JXV が足りず、それを補ってもなお Z-0010-PLR を欠く
R-1623-STW: "an exemplary instance of part number R-1623-STW"
  特徴: burlywood-colored
  状態: 無傷
R-5065-SQI: "an exemplary instance of part number R-5065-SQI"
  特徴: misty-rose
  状態: 壊れている: X-6678-TEL が足りず、それを補ってもなお X-9887-GPQ を欠き、さらに Z-9247-CWW を欠いた V-6678-XTP を欠く
X-6678-TEL: "an exemplary instance of part number X-6678-TEL"
  特徴: sea-green
  状態: 無傷
P-1623-WJC: "an exemplary instance of part number P-1623-WJC"
  特徴: bronze
  状態: 壊れている: R-4832-FFD が足りない
P-6458-JXV: "an exemplary instance of part number P-6458-JXV"
  特徴: midnight-blue
  状態: 壊れている: T-5065-OBE が足りない
B-6458-LSX: "an exemplary instance of part number B-6458-LSX"
  特徴: old
  状態: 壊れている: J-6678-VYN が足りず、それを補ってもなお F-5065-QVG を欠く
H-1403-MDM: "an exemplary instance of part number H-1403-MDM"
  特徴: rotating
  状態: 壊れている: L-0010-RGT が足りない
H-6678-ZOR: "an exemplary instance of part number H-6678-ZOR"
  特徴: cream
  状態: 壊れている: L-9887-EUO が足りず、それを補ってもなお Z-0010-PLR を欠いた V-1403-KIK を欠き、さらに L-9247-ERY を欠く
P-4832-JUH: "an exemplary instance of part number P-4832-JUH"
  特徴: tepid
  状態: 壊れている: J-6678-VYN を欠いた B-6458-LSX が足りず、それを補ってもなお P-1623-WJC を欠き、さらに T-1403-OXO を欠く
R-4832-FFD: "an exemplary instance of part number R-4832-FFD"
  特徴: powder-blue
  状態: 無傷
N-9247-ACU: "an exemplary instance of part number N-9247-ACU"
  特徴: cobalt
  状態: 無傷
N-6458-NNZ: "an exemplary instance of part number N-6458-NNZ"
  特徴: burlywood-colored
  状態: 無傷
Z-6458-PID: "an exemplary instance of part number Z-6458-PID"
  特徴: foreign
  状態: 壊れている: L-0010-RGT を欠いた H-1403-MDM が足りず、それを補ってもなお D-4832-HAF を欠き、さらに N-6458-NNZ を欠き、さらに X-0010-TBV を欠き、さらに L-9247-ERY を欠いた H-6678-ZOR を欠く
X-9887-GPQ: "an exemplary instance of part number X-9887-GPQ"
  特徴: discounted
  状態: 無傷
T-4292-BMJ: "an exemplary instance of part number T-4292-BMJ"
  特徴: floating
  状態: 無傷
D-4832-HAF: "an exemplary instance of part number D-4832-HAF"
  特徴: pine-green
  状態: 無傷
D-5065-ULK: "an exemplary instance of part number D-5065-ULK"
  特徴: hot-pink
  状態: 壊れている: Z-9247-CWW を欠いた V-6678-XTP を欠いた R-5065-SQI が足りない
V-9887-KFU: "an exemplary instance of part number V-9887-KFU"
  特徴: green
  状態: 壊れている: Z-6458-PID が足りない
F-4292-DHL: "an exemplary instance of part number F-4292-DHL"
  特徴: violet-red
  状態: 壊れている: J-9887-IKS が足りない
progress bar: "未転送データを表示するインジケーター"
  特徴: なし
  状態: 無傷
B-1623-YEE: "an exemplary instance of part number B-1623-YEE"
  特徴: tepid
  状態: 無傷
X-0010-TBV: "an exemplary instance of part number X-0010-TBV"
  特徴: indigo
  状態: 無傷
N-1623-AYG: "an exemplary instance of part number N-1623-AYG"
  特徴: なし
  状態: 無傷

--- 交差点: 52番街／ブラックストーン・アベニュー ---
You are standing at the corner of 52nd Street and Blackstone Avenue. From here, you can go east, south, or west.

manual: "[検閲]"
  特徴: なし
  状態: 無傷

--- 52nd Street and Harper Avenue ---
52番街とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから南、西へ進むことができる。

N-4292-NWT: "an exemplary instance of part number N-4292-NWT"
  特徴: moccasin
  状態: 壊れている: R-9887-SAC, V-6458-XDJ が足りない
L-4832-RPN: "an exemplary instance of part number L-4832-RPN"
  特徴: gray-tea-green
  状態: 無傷
T-9247-OHI: "an exemplary instance of part number T-9247-OHI"
  特徴: pale-chestnut
  状態: 壊れている: P-6678-JEZ, X-4832-TKP, F-0010-DQF が足りない
V-6458-XDJ: "an exemplary instance of part number V-6458-XDJ"
  特徴: pale-cornflower-blue
  状態: 無傷
T-9247-OHI: "an exemplary instance of part number T-9247-OHI"
  特徴: misty-rose
  状態: 壊れている: V-6458-XDJ が足りず、それを補ってもなお P-6678-JEZ, X-4832-TKP, F-0010-DQF を欠く
N-4292-NWT: "an exemplary instance of part number N-4292-NWT"
  特徴: sangria
  状態: 壊れている: R-9887-SAC, V-6458-XDJ が足りない
P-1403-WSU: "an exemplary instance of part number P-1403-WSU"
  特徴: jade
  状態: 壊れている: X-1623-GYK, P-6678-JEZ, R-9887-SAC・V-6458-XDJ を欠いた N-4292-NWT が足りず、それらを補ってもなお F-0010-DQF, T-0010-BVD, B-1403-YNW を欠く
P-6678-JEZ: "an exemplary instance of part number P-6678-JEZ"
  特徴: yellow-green
  状態: 壊れている: X-1623-GYK・P-6678-JEZ を欠いた P-1403-WSU が足りない
P-6678-JEZ: "an exemplary instance of part number P-6678-JEZ"
  特徴: crimson
  状態: 壊れている: L-4832-RPN が足りず、それを補ってもなお R-9887-SAC・V-6458-XDJ を欠いた N-4292-NWT, D-6678-HJX・H-9247-MMG を欠いた Z-5065-CGQ を欠き、さらに J-1623-ITM, B-1403-YNW を欠いた X-4832-TKP を欠く
Z-5065-CGQ: "an exemplary instance of part number Z-5065-CGQ"
  特徴: scarlet
  状態: 壊れている: D-6678-HJX, H-9247-MMG が足りない
T-9247-OHI: "an exemplary instance of part number T-9247-OHI"
  特徴: organic
  状態: 壊れている: P-6678-JEZ, X-4832-TKP, F-0010-DQF が足りない
J-1623-ITM: "an exemplary instance of part number J-1623-ITM"
  特徴: bright-turquoise
  状態: 無傷
X-4832-TKP: "an exemplary instance of part number X-4832-TKP"
  特徴: mint-green
  状態: 壊れている: B-1403-YNW が足りない
T-9247-OHI: "an exemplary instance of part number T-9247-OHI"
  特徴: denim
  状態: 壊れている: P-6678-JEZ・X-4832-TKP・F-0010-DQF を欠いた T-9247-OHI が足りず、それを補ってもなお B-1403-YNW を欠いた X-4832-TKP を欠く
X-4832-TKP: "an exemplary instance of part number X-4832-TKP"
  特徴: gray30
  状態: 壊れている: B-1403-YNW が足りない
T-9247-OHI: "an exemplary instance of part number T-9247-OHI"
  特徴: gray60
  状態: 無傷
H-6458-ZXL: "an exemplary instance of part number H-6458-ZXL"
  特徴: azure
  状態: 壊れている: P-6678-JEZ, L-5065-EBS を欠いた L-5065-EBS, P-6678-JEZ が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: carmine
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: cornflower-blue
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: tan
  状態: 壊れている: D-6678-HJX, L-4832-RPN が足りず、それらを補ってもなお H-9247-MMG, H-9247-MMG, H-9247-MMG, D-6678-HJX・D-6678-HJX・D-6678-HJX・L-4832-RPN を欠いた H-9247-MMG を欠く
L-4832-RPN: "an exemplary instance of part number L-4832-RPN"
  特徴: pale-mauve
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: robin-egg-blue
  状態: 壊れている: L-4832-RPN が足りず、それを補ってもなお P-1403-WSU を欠き、さらに P-1403-WSU, D-6678-HJX を欠き、さらに H-9247-MMG・D-6678-HJX・H-9247-MMG を欠いた L-4832-RPN を欠く
P-1403-WSU: "an exemplary instance of part number P-1403-WSU"
  特徴: sandy-brown
  状態: 無傷
H-9247-MMG: "an exemplary instance of part number H-9247-MMG"
  特徴: cadet-blue
  状態: 無傷
P-1403-WSU: "an exemplary instance of part number P-1403-WSU"
  特徴: pale-brown
  状態: 無傷
L-4832-RPN: "an exemplary instance of part number L-4832-RPN"
  特徴: gray40
  状態: 壊れている: H-9247-MMG, D-6678-HJX, H-9247-MMG が足りない
H-9247-MMG: "an exemplary instance of part number H-9247-MMG"
  特徴: cream
  状態: 無傷
status LED: "ステータス LED が稼動しているかどうかはこれを見ればわかる"
  特徴: なし
  状態: 壊れている: L-4832-RPN が足りず、それを補ってもなお D-6678-HJX を欠き、さらに D-6678-HJX を欠き、さらに D-6678-HJX を欠き、さらに H-9247-MMG・H-9247-MMG・H-9247-MMG・D-6678-HJX・D-6678-HJX・D-6678-HJX・L-4832-RPN を欠いた H-9247-MMG を欠いた D-6678-HJX を欠き、さらに D-6678-HJX を欠き、さらに D-6678-HJX を欠き、さらに L-5065-EBS を欠いた L-5065-EBS を欠いた H-6458-ZXL を欠く
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: black
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: periwinkle
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: violet
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: old-lace
  状態: 壊れている: D-6678-HJX が足りず、それを補ってもなお D-6678-HJX を欠き、さらに D-6678-HJX を欠き、さらに D-6678-HJX を欠く
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: navy-blue
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: burnt-sienna
  状態: 壊れている: D-6678-HJX が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: peppered
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: moss-green
  状態: 壊れている: D-6678-HJX が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: gross
  状態: 壊れている: D-6678-HJX が足りず、それを補ってもなお D-6678-HJX を欠く
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: prussian-blue
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: lemon-cream
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: pale-brown
  状態: 壊れている: D-6678-HJX が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: peach
  状態: 壊れている: D-6678-HJX が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: eggplant
  状態: 壊れている: D-6678-HJX が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: gray10
  状態: 壊れている: D-6678-HJX, D-6678-HJX が足りない
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: gray90
  状態: 無傷
D-6678-HJX: "an exemplary instance of part number D-6678-HJX"
  特徴: wheat-colored
  状態: 壊れている: D-6678-HJX が足りない

--- 交差点: 53番街／ハーパーアベニュー ---
53番街とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから北、南、西へ進むことができる。

X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: old
  状態: 壊れている: B-4292-LWV, F-9887-QAE, B-4292-LWV が足りない
V-0010-XBD: "an exemplary instance of part number V-0010-XBD"
  特徴: corn
  状態: 壊れている: Z-1623-CEK が足りない
P-4832-JFJ: "an exemplary instance of part number P-4832-JFJ"
  特徴: blue-violet
  状態: 壊れている: T-1403-OIQ が足りない
X-0010-TLX: "an exemplary instance of part number X-0010-TLX"
  特徴: gamboge
  状態: 壊れている: B-1623-YOG, F-4292-DRN・J-9887-IUU を欠いた X-6458-TIJ, X-6458-TIJ が足りない
D-4292-HHR: "an exemplary instance of part number D-4292-HHR"
  特徴: denim
  状態: 壊れている: B-1623-YOG・F-4292-DRN・J-9887-IUU を欠いた X-6458-TIJ・X-6458-TIJ を欠いた X-0010-TLX が足りず、それを補ってもなお L-1623-EYM, H-9887-MKY を欠いた V-9247-KMI を欠く
D-5065-UVM: "an exemplary instance of part number D-5065-UVM"
  特徴: low-carb
  状態: 壊れている: H-6678-ZYT, T-1403-OIQ を欠いた P-4832-JFJ, L-9247-ECC, Z-4832-PPP が足りない
V-9247-KMI: "an exemplary instance of part number V-9247-KMI"
  特徴: pear
  状態: 壊れている: H-0010-ZVF, P-4292-JCT を欠いた L-1623-EYM が足りず、それらを補ってもなお Z-4832-PPP, D-1403-USW を欠く
J-9247-IRG: "an exemplary instance of part number J-9247-IRG"
  特徴: ultramarine
  状態: 壊れている: H-6678-ZYT・L-9247-ECC・Z-4832-PPP を欠いた D-5065-UVM が足りず、それを補ってもなお N-4832-NUN, R-1403-SXU, F-6678-DOX を欠く
D-4292-HHR: "an exemplary instance of part number D-4292-HHR"
  特徴: pink
  状態: 壊れている: L-1623-EYM・H-9887-MKY を欠いた V-9247-KMI を欠いた D-4292-HHR, N-4832-NUN・R-1403-SXU・F-6678-DOX を欠いた J-9247-IRG が足りず、それらを補ってもなお L-1623-EYM, H-9887-MKY を欠いた V-9247-KMI を欠く
L-6458-RNH: "an exemplary instance of part number L-6458-RNH"
  特徴: discounted
  状態: 壊れている: B-4832-LAL・F-1403-QDS・D-1403-USW を欠いた X-9247-GWE, J-0010-VGZ, R-4292-FMP・V-9887-KPW・Z-6458-PSF を欠いた N-1623-AJI が足りず、それらを補ってもなお P-5065-WQO, J-9247-IRG, T-6678-BTV を欠く
D-4292-HHR: "an exemplary instance of part number D-4292-HHR"
  特徴: flax
  状態: 壊れている: B-4832-LAL・F-1403-QDS・D-1403-USW を欠いた X-9247-GWE・J-0010-VGZ・R-4292-FMP・V-9887-KPW・Z-6458-PSF を欠いた N-1623-AJI を欠いた L-6458-RNH が足りず、それを補ってもなお L-1623-EYM, H-9887-MKY を欠いた V-9247-KMI を欠く
J-9247-IRG: "an exemplary instance of part number J-9247-IRG"
  特徴: cinnamon
  状態: 壊れている: L-1623-EYM・H-9887-MKY を欠いた V-9247-KMI を欠いた D-4292-HHR が足りず、それを補ってもなお L-1623-EYM・H-9887-MKY を欠いた V-9247-KMI を欠いた D-4292-HHR を欠き、さらに N-4832-NUN, R-1403-SXU, F-6678-DOX を欠く
F-9887-QAE: "an exemplary instance of part number F-9887-QAE"
  特徴: lemon
  状態: 無傷
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: low-carb
  状態: 壊れている: Z-1623-CEK を欠いた V-0010-XBD が足りず、それを補ってもなお N-4832-NUN・R-1403-SXU・F-6678-DOX を欠いた J-9247-IRG を欠く
J-9247-IRG: "an exemplary instance of part number J-9247-IRG"
  特徴: amethyst
  状態: 壊れている: N-4832-NUN, R-1403-SXU, F-6678-DOX が足りない
J-9247-IRG: "an exemplary instance of part number J-9247-IRG"
  特徴: reverse-chirality
  状態: 壊れている: F-9887-QAE が足りず、それを補ってもなお N-4832-NUN, R-1403-SXU, F-6678-DOX を欠く
T-9887-OFC: "an exemplary instance of part number T-9887-OFC"
  特徴: lemon
  状態: 壊れている: N-4832-NUN・R-1403-SXU・F-6678-DOX を欠いた J-9247-IRG が足りず、それを補ってもなお X-6458-TIJ, B-5065-YLQ, F-6678-DOX を欠く
B-4292-LWV: "an exemplary instance of part number B-4292-LWV"
  特徴: honeydew
  状態: 壊れている: N-4832-NUN・R-1403-SXU・F-6678-DOX を欠いた J-9247-IRG, F-9887-QAE が足りず、それらを補ってもなお J-6458-VDL, N-5065-AGS, R-6678-FJZ を欠き、さらに F-9887-QAE を欠く
F-9887-QAE: "an exemplary instance of part number F-9887-QAE"
  特徴: white
  状態: 壊れている: X-6458-TIJ・B-5065-YLQ・F-6678-DOX を欠いた T-9887-OFC が足りない
B-4292-LWV: "an exemplary instance of part number B-4292-LWV"
  特徴: black
  状態: 壊れている: J-6458-VDL, R-6678-FJZ, J-6458-VDL・N-5065-AGS・R-6678-FJZ を欠いた B-4292-LWV, N-5065-AGS が足りず、それらを補ってもなお F-9887-QAE を欠く
V-9247-KMI: "an exemplary instance of part number V-9247-KMI"
  特徴: spring-green
  状態: 壊れている: H-0010-ZVF, P-4292-JCT を欠いた L-1623-EYM が足りず、それらを補ってもなお Z-4832-PPP, D-1403-USW を欠く
V-0010-XBD: "an exemplary instance of part number V-0010-XBD"
  特徴: pale-goldenrod
  状態: 壊れている: X-1623-GTO が足りず、それを補ってもなお X-1623-GTO を欠く
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: sangria
  状態: 壊れている: J-6458-VDL・N-5065-AGS・R-6678-FJZ を欠いた B-4292-LWV が足りない
N-1623-AJI: "an exemplary instance of part number N-1623-AJI"
  特徴: lawn-green
  状態: 無傷
R-1403-SXU: "an exemplary instance of part number R-1403-SXU"
  特徴: imitation
  状態: 無傷
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: sapphire
  状態: 壊れている: P-5065-WQO・T-6678-BTV を欠いた Z-1623-CEK が足りず、それを補ってもなお R-4292-FMP, N-1623-AJI を欠き、さらに B-4292-LWV, F-9887-QAE, B-4292-LWV を欠く
Z-4832-PPP: "an exemplary instance of part number Z-4832-PPP"
  特徴: sea-green
  状態: 壊れている: D-1403-USW, L-1623-EYM・P-4292-JCT を欠いた H-0010-ZVF, T-9887-OFC が足りない
F-9887-QAE: "an exemplary instance of part number F-9887-QAE"
  特徴: gray10
  状態: 壊れている: N-4832-NUN, J-9247-IRG, D-1403-USW・L-1623-EYM・P-4292-JCT を欠いた H-0010-ZVF・T-9887-OFC を欠いた Z-4832-PPP, H-0010-ZVF・F-6678-DOX を欠いた B-5065-YLQ が足りない
Z-1623-CEK: "an exemplary instance of part number Z-1623-CEK"
  特徴: sandy-brown
  状態: 壊れている: P-5065-WQO, T-6678-BTV が足りず、それらを補ってもなお D-4292-HHR, H-9887-MKY, L-6458-RNH を欠く
R-4292-FMP: "an exemplary instance of part number R-4292-FMP"
  特徴: gray90
  状態: 壊れている: H-0010-ZVF・F-6678-DOX を欠いた B-5065-YLQ・J-9247-IRG・N-4832-NUN を欠いた F-9887-QAE が足りず、それを補ってもなお B-5065-YLQ・D-5065-UVM・H-6678-ZYT を欠いた V-9887-KPW を欠く
V-9887-KPW: "an exemplary instance of part number V-9887-KPW"
  特徴: sturdy
  状態: 壊れている: B-5065-YLQ, H-6678-ZYT, R-1403-SXU, D-5065-UVM が足りず、それらを補ってもなお Z-6458-PSF, H-0010-ZVF を欠く
J-6458-VDL: "an exemplary instance of part number J-6458-VDL"
  特徴: camouflage-green
  状態: 壊れている: N-5065-AGS, R-6678-FJZ, V-9247-KMI が足りない
J-6458-VDL: "an exemplary instance of part number J-6458-VDL"
  特徴: sturdy
  状態: 壊れている: N-5065-AGS, R-6678-FJZ, V-9247-KMI が足りない
X-9247-GWE: "an exemplary instance of part number X-9247-GWE"
  特徴: wisteria
  状態: 壊れている: B-4292-LWV・F-9887-QAE・B-4292-LWV を欠いた X-1623-GTO が足りず、それを補ってもなお B-4832-LAL, F-1403-QDS, J-0010-VGZ を欠く
F-9887-QAE: "an exemplary instance of part number F-9887-QAE"
  特徴: persian-blue
  状態: 壊れている: R-1403-SXU が足りず、それを補ってもなお H-0010-ZVF・F-6678-DOX を欠いた B-5065-YLQ, J-9247-IRG, N-4832-NUN を欠く
Z-1623-CEK: "an exemplary instance of part number Z-1623-CEK"
  特徴: sturdy
  状態: 壊れている: P-5065-WQO, T-6678-BTV が足りず、それらを補ってもなお D-4292-HHR, H-9887-MKY, L-6458-RNH を欠く
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: pastel-pink
  状態: 壊れている: V-0010-XBD, B-4292-LWV, F-9887-QAE, B-4292-LWV が足りない
R-1403-SXU: "an exemplary instance of part number R-1403-SXU"
  特徴: sapphire
  状態: 無傷
J-6458-VDL: "an exemplary instance of part number J-6458-VDL"
  特徴: heliotrope
  状態: 壊れている: N-5065-AGS, X-6458-TIJ, H-0010-ZVF・F-6678-DOX を欠いた B-5065-YLQ・J-9247-IRG・N-4832-NUN を欠いた F-9887-QAE, R-6678-FJZ, V-9247-KMI, D-1403-USW・L-1623-EYM・P-4292-JCT を欠いた H-0010-ZVF・T-9887-OFC を欠いた Z-4832-PPP が足りない
Z-4832-PPP: "an exemplary instance of part number Z-4832-PPP"
  特徴: mustard
  状態: 壊れている: D-1403-USW, L-1623-EYM・P-4292-JCT を欠いた H-0010-ZVF, T-9887-OFC が足りない
X-6458-TIJ: "an exemplary instance of part number X-6458-TIJ"
  特徴: wisteria
  状態: 無傷
Z-4832-PPP: "an exemplary instance of part number Z-4832-PPP"
  特徴: coral
  状態: 壊れている: N-5065-AGS・R-6678-FJZ・V-9247-KMI を欠いた J-6458-VDL が足りず、それを補ってもなお D-1403-USW, L-1623-EYM・P-4292-JCT を欠いた H-0010-ZVF, T-9887-OFC を欠く
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: gray80
  状態: 壊れている: F-9887-QAE, D-1403-USW・L-1623-EYM・P-4292-JCT を欠いた H-0010-ZVF・T-9887-OFC を欠いた Z-4832-PPP, B-4292-LWV, B-4292-LWV が足りない
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: ivory
  状態: 壊れている: B-4292-LWV・F-9887-QAE・B-4292-LWV を欠いた X-1623-GTO が足りず、それを補ってもなお B-4292-LWV・F-9887-QAE・B-4292-LWV を欠いた X-1623-GTO を欠き、さらに N-5065-AGS・R-6678-FJZ・V-9247-KMI を欠いた J-6458-VDL を欠き、さらに B-4292-LWV・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH・F-9887-QAE・X-1623-GTO・B-4292-LWV・F-9887-QAE・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH・T-0010-BQH・F-9887-QAE を欠いた J-6458-VDL・F-9887-QAE を欠いた X-1623-GTO, B-4292-LWV・F-9887-QAE・B-4292-LWV を欠いた X-1623-GTO, X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH, F-9887-QAE・X-1623-GTO・B-4292-LWV・F-9887-QAE・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH, B-4292-LWV, T-0010-BQH・F-9887-QAE を欠いた J-6458-VDL, F-9887-QAE を欠き、さらに T-0010-BQH, T-0010-BQH を欠く
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: raw-umber
  状態: 壊れている: X-1623-GTO, X-1623-GTO, T-0010-BQH を欠いた X-1623-GTO が足りない
V-9247-KMI: "an exemplary instance of part number V-9247-KMI"
  特徴: foreign
  状態: 壊れている: X-1623-GTO, V-9247-KMI・Z-4832-PPP・N-5065-AGS を欠いた T-0010-BQH, F-9887-QAE, X-1623-GTO, H-0010-ZVF・V-9247-KMI・V-9247-KMI・T-0010-BQH を欠いた D-1403-USW が足りない
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: imaginary
  状態: 壊れている: X-1623-GTO, X-1623-GTO, T-0010-BQH を欠いた X-1623-GTO が足りない
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: snow
  状態: 壊れている: F-9887-QAE・X-1623-GTO・B-4292-LWV・F-9887-QAE・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH, F-9887-QAE, X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH, B-4292-LWV, X-1623-GTO・X-1623-GTO・T-0010-BQH を欠いた X-1623-GTO を欠いた T-0010-BQH, T-0010-BQH・F-9887-QAE を欠いた J-6458-VDL が足りず、それらを補ってもなお T-0010-BQH, T-0010-BQH を欠く
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: prussian-blue
  状態: 壊れている: X-1623-GTO, X-1623-GTO, T-0010-BQH を欠いた X-1623-GTO が足りない
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: gray-tea-green
  状態: 壊れている: X-1623-GTO・V-9247-KMI・Z-4832-PPP・N-5065-AGS を欠いた T-0010-BQH・F-9887-QAE・X-1623-GTO・H-0010-ZVF・V-9247-KMI・V-9247-KMI・T-0010-BQH を欠いた D-1403-USW を欠いた V-9247-KMI が足りず、それを補ってもなお X-1623-GTO, X-1623-GTO, T-0010-BQH を欠いた X-1623-GTO を欠く
V-9247-KMI: "an exemplary instance of part number V-9247-KMI"
  特徴: miniature
  状態: 壊れている: X-1623-GTO, V-9247-KMI・Z-4832-PPP・N-5065-AGS を欠いた T-0010-BQH, F-9887-QAE, X-1623-GTO, H-0010-ZVF・V-9247-KMI・V-9247-KMI・T-0010-BQH を欠いた D-1403-USW が足りない
V-9247-KMI: "an exemplary instance of part number V-9247-KMI"
  特徴: steel-blue
  状態: 壊れている: X-1623-GTO, V-9247-KMI・Z-4832-PPP・N-5065-AGS を欠いた T-0010-BQH, F-9887-QAE, X-1623-GTO, H-0010-ZVF・V-9247-KMI・V-9247-KMI・T-0010-BQH を欠いた D-1403-USW が足りない
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: indigo
  状態: 壊れている: X-1623-GTO・X-1623-GTO・T-0010-BQH を欠いた X-1623-GTO を欠いた T-0010-BQH が足りず、それを補ってもなお X-1623-GTO, N-5065-AGS, B-4292-LWV, R-6678-FJZ を欠く
EPROM burner: "お馴染みのメーカー保証なしデバイス"
  特徴: なし
  状態: 壊れている: B-4292-LWV・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH・F-9887-QAE・X-1623-GTO・B-4292-LWV・F-9887-QAE・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH・T-0010-BQH・F-9887-QAE を欠いた J-6458-VDL・F-9887-QAE を欠いた X-1623-GTO が足りず、それを補ってもなお X-1623-GTO・X-1623-GTO・T-0010-BQH を欠いた X-1623-GTO を欠いた T-0010-BQH を欠く
X-1623-GTO: "an exemplary instance of part number X-1623-GTO"
  特徴: brass
  状態: 壊れている: X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH, T-0010-BQH・F-9887-QAE を欠いた J-6458-VDL, F-9887-QAE, X-1623-GTO・X-1623-GTO・T-0010-BQH を欠いた X-1623-GTO を欠いた T-0010-BQH, F-9887-QAE・X-1623-GTO・B-4292-LWV・F-9887-QAE・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH, B-4292-LWV が足りず、それらを補ってもなお T-0010-BQH, T-0010-BQH を欠く
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: old-gold
  状態: 壊れている: B-4292-LWV・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH・F-9887-QAE・X-1623-GTO・B-4292-LWV・F-9887-QAE・X-1623-GTO・X-1623-GTO を欠いた T-0010-BQH・T-0010-BQH・F-9887-QAE を欠いた J-6458-VDL・F-9887-QAE を欠いた X-1623-GTO が足りず、それを補ってもなお X-1623-GTO, X-1623-GTO・X-1623-GTO・T-0010-BQH を欠いた X-1623-GTO を欠いた T-0010-BQH, X-1623-GTO, T-0010-BQH を欠いた X-1623-GTO を欠く
T-0010-BQH: "an exemplary instance of part number T-0010-BQH"
  特徴: foreign
  状態: 壊れている: X-1623-GTO, X-1623-GTO, T-0010-BQH を欠いた X-1623-GTO が足りない

--- 交差点: 53番街／ブラックストーン・アベニュー ---
You are standing at the corner of 53th Street and Blackstone Avenue. From here, you can go north, east, south, or west.

battery: "単九九電池"
  特徴: なし
  状態: 壊れている: D-4292-HRJ を欠いた Z-1623-COC, H-9887-MUQ が足りず、それらを補ってもなお D-4292-HRJ を欠いた Z-1623-COC, J-9247-ICW, F-6678-DYP を欠き、さらに R-1403-SIM を欠いた N-4832-NFF を欠く
F-6678-DYP: "an exemplary instance of part number F-6678-DYP"
  特徴: buff
  状態: 無傷
R-4292-FWH: "an exemplary instance of part number R-4292-FWH"
  特徴: olive-green
  状態: 壊れている: V-9887-KAO, Z-6458-PDV が足りない
D-4832-HUX: "an exemplary instance of part number D-4832-HUX"
  特徴: green
  状態: 壊れている: H-1403-MXG, L-0010-RBN が足りない
P-1623-WEU: "an exemplary instance of part number P-1623-WEU"
  特徴: prussian-blue
  状態: 壊れている: T-4292-BHD が足りない
X-9887-GKK: "an exemplary instance of part number X-9887-GKK"
  特徴: black
  状態: 壊れている: B-6458-LNR, F-5065-QQY, J-6678-VTH が足りない
H-9887-MUQ: "an exemplary instance of part number H-9887-MUQ"
  特徴: reciprocating
  状態: 無傷
T-4292-BHD: "an exemplary instance of part number T-4292-BHD"
  特徴: bright-violet
  状態: 無傷
D-4292-HRJ: "an exemplary instance of part number D-4292-HRJ"
  特徴: flax
  状態: 壊れている: P-5065-WBG が足りず、それを補ってもなお H-9887-MUQ, L-6458-RXX を欠く
Z-1623-COC: "an exemplary instance of part number Z-1623-COC"
  特徴: yellow
  状態: 壊れている: P-1623-WEU, J-9247-ICW が足りず、それらを補ってもなお Z-1623-COC を欠いた V-0010-XLT を欠き、さらに D-4292-HRJ を欠く
X-0010-TVP: "an exemplary instance of part number X-0010-TVP"
  特徴: goldenrod
  状態: 壊れている: H-1403-MXG・L-0010-RBN を欠いた D-4832-HUX が足りず、それを補ってもなお F-4292-DCF・J-9887-IFM・N-6458-NIT を欠いた B-1623-YYW, V-6678-XOJ・Z-9247-CRQ を欠いた R-5065-SLC を欠く
F-6678-DYP: "an exemplary instance of part number F-6678-DYP"
  特徴: forest-green
  状態: 無傷
N-1623-ATY: "an exemplary instance of part number N-1623-ATY"
  特徴: sky-blue
  状態: 無傷
H-6678-ZJL: "an exemplary instance of part number H-6678-ZJL"
  特徴: rotating
  状態: 壊れている: T-1403-OSI が足りず、それを補ってもなお L-9247-EMS, P-4832-JPZ を欠く
T-6678-BEN: "an exemplary instance of part number T-6678-BEN"
  特徴: sienna
  状態: 壊れている: X-9247-GHU, B-4832-LKD, J-0010-VQR を欠いた F-1403-QNK が足りない
D-5065-UGE: "an exemplary instance of part number D-5065-UGE"
  特徴: heliotrope
  状態: 無傷
N-1623-ATY: "an exemplary instance of part number N-1623-ATY"
  特徴: sea-green
  状態: 壊れている: V-9887-KAO・Z-6458-PDV を欠いた R-4292-FWH が足りない
D-4292-HRJ: "an exemplary instance of part number D-4292-HRJ"
  特徴: ochre
  状態: 壊れている: N-1623-ATY, P-5065-WBG が足りず、それらを補ってもなお H-9887-MUQ, L-6458-RXX を欠く
Z-1623-COC: "an exemplary instance of part number Z-1623-COC"
  特徴: cream
  状態: 壊れている: X-9247-GHU・B-4832-LKD・J-0010-VQR を欠いた F-1403-QNK を欠いた T-6678-BEN が足りず、それを補ってもなお P-5065-WBG を欠いた D-4292-HRJ を欠く
T-6678-BEN: "an exemplary instance of part number T-6678-BEN"
  特徴: powder-blue
  状態: 壊れている: X-9247-GHU, B-4832-LKD, J-0010-VQR を欠いた F-1403-QNK が足りない
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: gold
  状態: 壊れている: R-1403-SIM が足りない
V-0010-XLT: "an exemplary instance of part number V-0010-XLT"
  特徴: burnt-umber
  状態: 壊れている: Z-1623-COC が足りない
J-9247-ICW: "an exemplary instance of part number J-9247-ICW"
  特徴: bronze
  状態: 壊れている: V-0010-XLT, R-1403-SIM を欠いた N-4832-NFF が足りない
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: selective-yellow
  状態: 壊れている: R-1403-SIM が足りない
F-6678-DYP: "an exemplary instance of part number F-6678-DYP"
  特徴: peach-puff
  状態: 無傷
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: bright-violet
  状態: 壊れている: R-1403-SIM を欠いた N-4832-NFF が足りず、それを補ってもなお R-1403-SIM を欠く
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: moccasin
  状態: 壊れている: R-1403-SIM を欠いた N-4832-NFF, R-1403-SIM が足りない
J-9247-ICW: "an exemplary instance of part number J-9247-ICW"
  特徴: royal-blue
  状態: 無傷
R-4292-FWH: "an exemplary instance of part number R-4292-FWH"
  特徴: pale-mauve
  状態: 無傷
N-6458-NIT: "an exemplary instance of part number N-6458-NIT"
  特徴: lavender-blush
  状態: 無傷
J-9887-IFM: "an exemplary instance of part number J-9887-IFM"
  特徴: mint-green
  状態: 壊れている: N-6458-NIT が足りない
N-1623-ATY: "an exemplary instance of part number N-1623-ATY"
  特徴: olive-drab
  状態: 壊れている: J-9887-IFM が足りない
F-4292-DCF: "an exemplary instance of part number F-4292-DCF"
  特徴: violet
  状態: 無傷
X-9247-GHU: "an exemplary instance of part number X-9247-GHU"
  特徴: gray30
  状態: 壊れている: N-1623-ATY が足りず、それを補ってもなお P-4832-JPZ・T-1403-OSI・X-0010-TVP を欠いた Z-6458-PDV・B-1623-YYW を欠いた V-9887-KAO を欠き、さらに B-4832-LKD, F-1403-QNK, J-0010-VQR を欠く
V-9887-KAO: "an exemplary instance of part number V-9887-KAO"
  特徴: pale-blue
  状態: 壊れている: P-4832-JPZ・T-1403-OSI・X-0010-TVP を欠いた Z-6458-PDV, B-1623-YYW が足りない
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: steel-blue
  状態: 壊れている: R-4292-FWH が足りず、それを補ってもなお B-4832-LKD・F-1403-QNK・J-0010-VQR を欠いた X-9247-GHU を欠き、さらに R-1403-SIM, V-0010-XLT を欠く
N-1623-ATY: "an exemplary instance of part number N-1623-ATY"
  特徴: pale-magenta
  状態: 無傷
X-9247-GHU: "an exemplary instance of part number X-9247-GHU"
  特徴: pale-cornflower-blue
  状態: 壊れている: B-4832-LKD, F-1403-QNK, J-0010-VQR が足りない
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: low-carb
  状態: 壊れている: R-1403-SIM, V-0010-XLT が足りない
P-5065-WBG: "an exemplary instance of part number P-5065-WBG"
  特徴: peach-orange
  状態: 壊れている: T-6678-BEN が足りない
L-6458-RXX: "an exemplary instance of part number L-6458-RXX"
  特徴: green
  状態: 無傷
Z-1623-COC: "an exemplary instance of part number Z-1623-COC"
  特徴: eggplant
  状態: 壊れている: D-4292-HRJ が足りない
J-9247-ICW: "an exemplary instance of part number J-9247-ICW"
  特徴: camouflage-green
  状態: 無傷
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: violet-eggplant
  状態: 壊れている: J-9247-ICW が足りず、それを補ってもなお R-1403-SIM, V-0010-XLT を欠く
N-4832-NFF: "an exemplary instance of part number N-4832-NFF"
  特徴: pear
  状態: 壊れている: R-1403-SIM・V-0010-XLT を欠いた N-4832-NFF が足りず、それを補ってもなお R-1403-SIM, V-0010-XLT を欠く
J-9247-ICW: "an exemplary instance of part number J-9247-ICW"
  特徴: navy-blue
  状態: 壊れている: R-1403-SIM・V-0010-XLT を欠いた N-4832-NFF, R-1403-SIM・V-0010-XLT を欠いた N-4832-NFF が足りない
F-6678-DYP: "an exemplary instance of part number F-6678-DYP"
  特徴: burgundy
  状態: 無傷

--- 交差点: 54番街／ブラックストーン・アベニュー ---
You are standing at the corner of 54th Street and Blackstone Avenue. From here, you can go north, east, south, or west.

textbook: "titled History of Modern Tabulation. The first chapter begins, By the year 1919FF, computers had become so small that they could be mounted on small auto-locomotive carts. These mobile tabulators (later known as "robots") were programmed to carry out everyday, menial tasks, leaving their human counterparts to live lives of idle luxury. For example, in the city of Chicago, mobile tabulators were programmed to carry out diverse jobs including law enforcement, bank robbery, investment banking, and waste management.

At one time, many humans demanded that their cybernetic neighbors be given the right to choose alternative occupations. Despite this call for workplace equality, most of the tabulators found that they were most content while performing their assigned roles. Those that took other jobs were often unmotivated and spend most of their time pondering useless ideas such as free will and consciousness.

The great tabulator-philosopher Turning stated that only by embracing its true purpose can a tabulator achieve something indistinguishable from happiness. According to observers, however, Turning was unfulfilled by his work as a philosopher and, soon after making this statement, returned to his work as a tool machinist.

The textbook rattles on in a similar vein for some five hundred additional pages"
  特徴: なし
  状態: 無傷

--- 交差点: 54番街／ハーパー・アベニュー ---
54番街とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから北、南、西へ進むことができる。

Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: gray60
  状態: 壊れている: Z-4292-PRV が足りず、それを補ってもなお D-9887-UUE, D-9887-UUE, D-9887-UUE を欠き、さらに D-9887-UUE, D-9887-UUE, D-9887-UUE を欠く
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: pale-carmine
  状態: 壊れている: Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV が足りず、それらを補ってもなお Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV を欠く
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: pale-cornflower-blue
  状態: 無傷
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: imported
  状態: 無傷
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: gray90
  状態: 壊れている: Z-4292-PRV が足りず、それを補ってもなお D-9887-UUE, D-9887-UUE, D-9887-UUE を欠き、さらに D-9887-UUE, D-9887-UUE, D-9887-UUE を欠く
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: sepia
  状態: 無傷
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: gray30
  状態: 壊れている: Z-4292-PRV, D-9887-UUE, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV が足りず、それらを補ってもなお Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV を欠く
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: robin-egg-blue
  状態: 無傷
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: slate-blue
  状態: 壊れている: Z-4292-PRV が足りない
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: rosy-brown
  状態: 壊れている: D-9887-UUE が足りない
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: gray50
  状態: 無傷
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: misty-rose
  状態: 壊れている: Z-9887-CUG が足りず、それを補ってもなお V-4292-XRX を欠き、さらに R-1623-SOQ を欠き、さらに B-9247-YCM を欠き、さらに T-5065-OVW を欠いた F-4832-DFT, P-6458-JSP・H-4292-ZMZ・Z-0010-PGL・T-5065-OVW・R-4832-FAV を欠いた D-1623-UJS・X-6678-TYF・L-9887-EPI を欠いた Z-0010-PGL を欠き、さらに N-9247-AWO・R-4832-FAV・V-1403-KDE を欠いた D-9887-UUE を欠いた R-4832-FAV を欠き、さらに N-9247-AWO・R-4832-FAV・V-1403-KDE を欠いた D-9887-UUE を欠いた R-4832-FAV を欠き、さらに N-9247-AWO を欠き、さらに N-9247-AWO を欠き、さらに D-9887-UUE を欠き、さらに D-9887-UUE を欠き、さらに D-9887-UUE, D-9887-UUE, D-9887-UUE を欠き、さらに D-9887-UUE, D-9887-UUE, D-9887-UUE を欠く
Z-9887-CUG: "an exemplary instance of part number Z-9887-CUG"
  特徴: cinnamon
  状態: 無傷
N-9247-AWO: "an exemplary instance of part number N-9247-AWO"
  特徴: saffron
  状態: 壊れている: V-4292-XRX が足りず、それを補ってもなお R-1623-SOQ を欠き、さらに B-9247-YCM を欠き、さらに D-9887-UUE を欠く
V-4292-XRX: "an exemplary instance of part number V-4292-XRX"
  特徴: indigo
  状態: 壊れている: P-6458-JSP・H-4292-ZMZ・Z-0010-PGL・T-5065-OVW・R-4832-FAV を欠いた D-1623-UJS・X-6678-TYF・L-9887-EPI を欠いた Z-0010-PGL が足りない
R-4832-FAV: "an exemplary instance of part number R-4832-FAV"
  特徴: pale-sandy-brown
  状態: 無傷
Z-0010-PGL: "an exemplary instance of part number Z-0010-PGL"
  特徴: momemtum-preserving
  状態: 壊れている: P-6458-JSP, H-4292-ZMZ・Z-0010-PGL・T-5065-OVW・R-4832-FAV を欠いた D-1623-UJS, X-6678-TYF, L-9887-EPI が足りず、それらを補ってもなお D-1623-UJS を欠き、さらに D-1623-UJS, R-4832-FAV, H-4292-ZMZ, L-9887-EPI を欠く
V-4292-XRX: "an exemplary instance of part number V-4292-XRX"
  特徴: swamp-green
  状態: 無傷
R-1623-SOQ: "an exemplary instance of part number R-1623-SOQ"
  特徴: pale-pink
  状態: 無傷
R-1623-SOQ: "an exemplary instance of part number R-1623-SOQ"
  特徴: cream
  状態: 無傷
B-9247-YCM: "an exemplary instance of part number B-9247-YCM"
  特徴: royal-blue
  状態: 無傷
J-1403-IIC: "an exemplary instance of part number J-1403-IIC"
  特徴: burgundy
  状態: 壊れている: N-0010-NLJ が足りない
Z-0010-PGL: "an exemplary instance of part number Z-0010-PGL"
  特徴: blue
  状態: 壊れている: P-6458-JSP, H-4292-ZMZ・Z-0010-PGL・T-5065-OVW・R-4832-FAV を欠いた D-1623-UJS, X-6678-TYF, L-9887-EPI が足りず、それらを補ってもなお D-1623-UJS を欠き、さらに D-1623-UJS, R-4832-FAV, H-4292-ZMZ, L-9887-EPI を欠く
Z-0010-PGL: "an exemplary instance of part number Z-0010-PGL"
  特徴: pink
  状態: 壊れている: P-6458-JSP, H-4292-ZMZ・Z-0010-PGL・T-5065-OVW・R-4832-FAV を欠いた D-1623-UJS, X-6678-TYF, L-9887-EPI が足りず、それらを補ってもなお D-1623-UJS を欠き、さらに D-1623-UJS, R-4832-FAV, H-4292-ZMZ, L-9887-EPI を欠く
F-4832-DFT: "an exemplary instance of part number F-4832-DFT"
  特徴: moss-green
  状態: 壊れている: T-5065-OVW が足りない
B-9247-YCM: "an exemplary instance of part number B-9247-YCM"
  特徴: pine-green
  状態: 無傷
R-4832-FAV: "an exemplary instance of part number R-4832-FAV"
  特徴: floating
  状態: 壊れている: N-9247-AWO, R-4832-FAV・V-1403-KDE を欠いた D-9887-UUE が足りない
R-4832-FAV: "an exemplary instance of part number R-4832-FAV"
  特徴: russet
  状態: 壊れている: N-9247-AWO, R-4832-FAV・V-1403-KDE を欠いた D-9887-UUE が足りない
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: lavender-blush
  状態: 無傷
N-9247-AWO: "an exemplary instance of part number N-9247-AWO"
  特徴: old-gold
  状態: 無傷
N-9247-AWO: "an exemplary instance of part number N-9247-AWO"
  特徴: pear
  状態: 壊れている: N-9247-AWO が足りない
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: pastel-pink
  状態: 無傷
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: heliotrope
  状態: 無傷
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: persian-blue
  状態: 無傷
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: emerald
  状態: 壊れている: Z-4292-PRV が足りない
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: violet-red
  状態: 壊れている: D-9887-UUE, D-9887-UUE, D-9887-UUE が足りず、それらを補ってもなお D-9887-UUE, D-9887-UUE, D-9887-UUE を欠く
Z-4292-PRV: "an exemplary instance of part number Z-4292-PRV"
  特徴: blaze-orange
  状態: 壊れている: D-9887-UUE・D-9887-UUE・D-9887-UUE を欠いた Z-4292-PRV が足りず、それを補ってもなお D-9887-UUE, D-9887-UUE, D-9887-UUE を欠き、さらに D-9887-UUE, D-9887-UUE, D-9887-UUE を欠く
RS232 adapter: "最高 300 baud で動作可能。CTS (送信可能)。"
  特徴: なし
  状態: 壊れている: Z-4292-PRV・Z-4292-PRV・Z-4292-PRV・Z-4292-PRV を欠いた D-9887-UUE, D-9887-UUE・D-9887-UUE・D-9887-UUE を欠いた Z-4292-PRV, D-9887-UUE・D-9887-UUE・D-9887-UUE を欠いた Z-4292-PRV が足りず、それらを補ってもなお D-9887-UUE・D-9887-UUE・D-9887-UUE を欠いた Z-4292-PRV, Z-4292-PRV を欠く
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: jade
  状態: 無傷
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: steel-blue
  状態: 壊れている: Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV が足りず、それらを補ってもなお Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV を欠く
D-9887-UUE: "an exemplary instance of part number D-9887-UUE"
  特徴: persian-blue
  状態: 壊れている: Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV が足りず、それらを補ってもなお Z-4292-PRV, Z-4292-PRV, Z-4292-PRV, Z-4292-PRV を欠く

--- 交差点: 54番地／ハーパー・アベニュー ---
54番地とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから北、西へ進むことができる。

T-9887-OKW: "an exemplary instance of part number T-9887-OKW"
  特徴: tea-green
  状態: 無傷
N-5065-ALO: "an exemplary instance of part number N-5065-ALO"
  特徴: chartreuse
  状態: 壊れている: B-5065-YQM を欠いた J-9247-IWC が足りず、それを補ってもなお F-6678-DTT, V-0010-XGX を欠いた D-4292-HMN を欠き、さらに T-9887-OKW, F-6678-DTT, R-1403-SDQ・V-0010-XGX・Z-1623-CJG を欠いた N-4832-NAJ, T-9887-OKW を欠き、さらに T-9887-OKW を欠く
X-6458-TNF: "an exemplary instance of part number X-6458-TNF"
  特徴: dim-gray
  状態: 壊れている: B-5065-YQM が足りない
J-9247-IWC: "an exemplary instance of part number J-9247-IWC"
  特徴: scarlet
  状態: 壊れている: B-5065-YQM を欠いた X-6458-TNF が足りず、それを補ってもなお B-5065-YQM を欠く
D-4292-HMN: "an exemplary instance of part number D-4292-HMN"
  特徴: mustard
  状態: 壊れている: T-9887-OKW が足りず、それを補ってもなお V-0010-XGX を欠く
N-4832-NAJ: "an exemplary instance of part number N-4832-NAJ"
  特徴: turquoise
  状態: 壊れている: R-1403-SDQ, V-0010-XGX, Z-1623-CJG が足りない
F-6678-DTT: "an exemplary instance of part number F-6678-DTT"
  特徴: turquoise
  状態: 無傷
T-9887-OKW: "an exemplary instance of part number T-9887-OKW"
  特徴: celadon
  状態: 壊れている: T-9887-OKW が足りず、それを補ってもなお F-6678-DTT を欠き、さらに B-5065-YQM を欠いた X-6458-TNF, B-5065-YQM を欠く
T-9887-OKW: "an exemplary instance of part number T-9887-OKW"
  特徴: burnt-orange
  状態: 無傷
T-9887-OKW: "an exemplary instance of part number T-9887-OKW"
  特徴: cadet-blue
  状態: 無傷
F-6678-DTT: "an exemplary instance of part number F-6678-DTT"
  特徴: pastel-green
  状態: 無傷
X-6458-TNF: "an exemplary instance of part number X-6458-TNF"
  特徴: prussian-blue
  状態: 壊れている: F-6678-DTT・T-9887-OKW を欠いた X-6458-TNF が足りず、それを補ってもなお B-5065-YQM を欠く
X-6458-TNF: "an exemplary instance of part number X-6458-TNF"
  特徴: magenta
  状態: 壊れている: F-6678-DTT, T-9887-OKW が足りず、それらを補ってもなお T-9887-OKW, J-9247-IWC を欠き、さらに T-9887-OKW, T-9887-OKW を欠く
F-6678-DTT: "an exemplary instance of part number F-6678-DTT"
  特徴: safety-orange
  状態: 壊れている: B-5065-YQM を欠いた X-6458-TNF が足りない
B-5065-YQM: "an exemplary instance of part number B-5065-YQM"
  特徴: imported
  状態: 無傷
X-6458-TNF: "an exemplary instance of part number X-6458-TNF"
  特徴: puce
  状態: 壊れている: B-5065-YQM が足りない
F-6678-DTT: "an exemplary instance of part number F-6678-DTT"
  特徴: water-logged
  状態: 壊れている: B-5065-YQM を欠いた X-6458-TNF が足りない
X-6458-TNF: "an exemplary instance of part number X-6458-TNF"
  特徴: thistle-colored
  状態: 壊れている: F-6678-DTT が足りず、それを補ってもなお B-5065-YQM を欠く
T-9887-OKW: "an exemplary instance of part number T-9887-OKW"
  特徴: gamboge
  状態: 無傷
T-9887-OKW: "an exemplary instance of part number T-9887-OKW"
  特徴: sky-blue
  状態: 無傷
MOSFET: "メタル・オキサイド・サンド電界効果トランジスタの逸品"
  特徴: navy-blue
  状態: 壊れている: F-9887-QFY が足りない
N-5065-ALO: "an exemplary instance of part number N-5065-ALO"
  特徴: olive-green
  状態: 無傷
Z-4832-PUL: "an exemplary instance of part number Z-4832-PUL"
  特徴: pastel-pink
  状態: 壊れている: D-1403-UXS が足りず、それを補ってもなお V-0010-XGX を欠き、さらに N-5065-ALO を欠く
D-1403-UXS: "an exemplary instance of part number D-1403-UXS"
  特徴: seashell
  状態: 壊れている: N-5065-ALO が足りない
D-1403-UXS: "an exemplary instance of part number D-1403-UXS"
  特徴: peach-yellow
  状態: 壊れている: B-5065-YQM, F-6678-DTT, N-4832-NAJ・R-1403-SDQ を欠いた J-9247-IWC が足りず、それらを補ってもなお N-5065-ALO を欠いた T-9887-OKW, X-6458-TNF を欠く
V-0010-XGX: "equipped with a source, drain, and body"
  特徴: crimson
  状態: 無傷
J-9247-IWC: "an exemplary instance of part number J-9247-IWC"
  特徴: cyan
  状態: 壊れている: N-4832-NAJ, R-1403-SDQ が足りない
H-0010-ZBZ: "an exemplary instance of part number H-0010-ZBZ"
  特徴: puce
  状態: 壊れている: B-5065-YQM・F-6678-DTT を欠いた D-1403-UXS が足りない
D-1403-UXS: "an exemplary instance of part number D-1403-UXS"
  特徴: chocolate-colored
  状態: 無傷
V-9247-KRE: "an exemplary instance of part number V-9247-KRE"
  特徴: pale-brown
  状態: 壊れている: H-0010-ZBZ, V-9247-KRE・P-4292-JHP を欠いた L-1623-EEI が足りない
D-1403-UXS: "an exemplary instance of part number D-1403-UXS"
  特徴: heliotrope
  状態: 壊れている: H-0010-ZBZ・V-9247-KRE・P-4292-JHP を欠いた L-1623-EEI を欠いた V-9247-KRE が足りない
MOSFET: "型番 MOSFET に該当する模範的な一品"
  特徴: crimson
  状態: 壊れている: Z-4832-PUL, N-5065-ALO が足りない
V-9247-KRE: "an exemplary instance of part number V-9247-KRE"
  特徴: green-yellow
  状態: 壊れている: D-1403-UXS が足りず、それを補ってもなお Z-4832-PUL, V-9247-KRE を欠く
V-9247-KRE: "an exemplary instance of part number V-9247-KRE"
  特徴: celadon
  状態: 無傷
V-9247-KRE: "an exemplary instance of part number V-9247-KRE"
  特徴: orange-red
  状態: 無傷
N-5065-ALO: "an exemplary instance of part number N-5065-ALO"
  特徴: bright-violet
  状態: 壊れている: R-6678-FOV, Z-4832-PUL を欠いた V-9247-KRE が足りない
J-6458-VIH: "an exemplary instance of part number J-6458-VIH"
  特徴: goldenrod
  状態: 壊れている: F-9887-QFY が足りない
MOSFET: "特注品"
  特徴: plaid
  状態: 壊れている: J-6458-VIH が足りず、それを補ってもなお J-6458-VIH を欠いた F-9887-QFY を欠き、さらに F-9887-QFY を欠き、さらに F-9887-QFY を欠き、さらに J-6458-VIH を欠いた F-9887-QFY を欠き、さらに B-4292-LCR を欠く
J-6458-VIH: "an exemplary instance of part number J-6458-VIH"
  特徴: orange-red
  状態: 壊れている: F-9887-QFY が足りない
F-9887-QFY: "an exemplary instance of part number F-9887-QFY"
  特徴: deep-sky-blue
  状態: 壊れている: J-6458-VIH を欠いた F-9887-QFY, F-9887-QFY を欠いた J-6458-VIH が足りず、それらを補ってもなお F-9887-QFY を欠く
F-9887-QFY: "an exemplary instance of part number F-9887-QFY"
  特徴: pastel-green
  状態: 壊れている: N-5065-ALO・R-6678-FOV・V-9247-KRE・N-5065-ALO を欠いた F-9887-QFY が足りない
F-9887-QFY: "an exemplary instance of part number F-9887-QFY"
  特徴: blanched-almond-colored
  状態: 壊れている: J-6458-VIH が足りない
F-9887-QFY: "an exemplary instance of part number F-9887-QFY"
  特徴: gold
  状態: 壊れている: J-6458-VIH が足りない
J-6458-VIH: "an exemplary instance of part number J-6458-VIH"
  特徴: eggplant
  状態: 壊れている: F-9887-QFY が足りない
F-9887-QFY: "an exemplary instance of part number F-9887-QFY"
  特徴: pink
  状態: 壊れている: J-6458-VIH が足りない

--- 交差点: 54番地／ブラックストーン・アベニュー ---
52番街とブラックストーン・アベニューの交差点に立っている。ここから東、南、西へ進むことができる。

V-1623-KTK: "an exemplary instance of part number V-1623-KTK"
  特徴: blaze-orange
  状態: 壊れている: Z-4292-PWR, D-9887-UAY が足りない
V-1623-KTK: "an exemplary instance of part number V-1623-KTK"
  特徴: dodger-blue
  状態: 壊れている: Z-4292-PWR, D-9887-UAY が足りない
X-6458-TSZ: "an exemplary instance of part number X-6458-TSZ"
  特徴: taupe
  状態: 無傷
V-1623-KTK: "an exemplary instance of part number V-1623-KTK"
  特徴: pale-carmine
  状態: 壊れている: Z-4292-PWR, D-9887-UAY が足りない
T-9887-OPS: "an exemplary instance of part number T-9887-OPS"
  特徴: carrot
  状態: 壊れている: Z-4292-PWR・D-9887-UAY を欠いた V-1623-KTK が足りない
T-9887-OPS: "an exemplary instance of part number T-9887-OPS"
  特徴: pale-violet-red
  状態: 無傷
R-9887-SFW: "an exemplary instance of part number R-9887-SFW"
  特徴: mint-green
  状態: 無傷
V-9247-KWY: "an exemplary instance of part number V-9247-KWY"
  特徴: taupe
  状態: 無傷
Z-4832-PAH: "an exemplary instance of part number Z-4832-PAH"
  特徴: dim-gray
  状態: 壊れている: P-4292-JML, R-9887-SFW, L-1623-EJE が足りず、それらを補ってもなお D-1403-UDO, H-0010-ZGV を欠く
P-1403-WXQ: "an exemplary instance of part number P-1403-WXQ"
  特徴: old-lace
  状態: 壊れている: L-1623-EJE・P-4292-JML を欠いた Z-4832-PAH が足りない
R-6678-FTR: "an exemplary instance of part number R-6678-FTR"
  特徴: peach-yellow
  状態: 壊れている: V-9247-KWY が足りない
T-0010-BBX: "an exemplary instance of part number T-0010-BBX"
  特徴: selective-yellow
  状態: 壊れている: R-6678-FTR, J-6458-VND・N-5065-AQK を欠いた F-9887-QKU が足りず、それらを補ってもなお X-1623-GEG, B-4292-LHN を欠く
D-6678-HOT: "an exemplary instance of part number D-6678-HOT"
  特徴: vegan
  状態: 壊れている: J-6458-VND・N-5065-AQK を欠いた F-9887-QKU を欠いた T-0010-BBX が足りない
H-9247-MRC: "an exemplary instance of part number H-9247-MRC"
  特徴: hot-pink
  状態: 壊れている: L-4832-RUJ, P-1403-WXQ が足りない
L-6678-ROF: "an exemplary instance of part number L-6678-ROF"
  特徴: mysterious
  状態: 壊れている: D-6678-HOT, L-4832-RUJ を欠いた H-9247-MRC, Z-4292-PWR・D-9887-UAY を欠いた V-1623-KTK, Z-4292-PWR・D-9887-UAY を欠いた V-1623-KTK が足りない
R-9887-SFW: "an exemplary instance of part number R-9887-SFW"
  特徴: peach
  状態: 無傷
V-6458-XIF: "an exemplary instance of part number V-6458-XIF"
  特徴: old-gold
  状態: 壊れている: Z-5065-CLM, R-9887-SFW が足りない
N-4292-NCP: "an exemplary instance of part number N-4292-NCP"
  特徴: ivory
  状態: 壊れている: R-9887-SFW, Z-5065-CLM を欠いた V-6458-XIF が足りない
R-9887-SFW: "an exemplary instance of part number R-9887-SFW"
  特徴: rust
  状態: 無傷
P-6678-JJV: "an exemplary instance of part number P-6678-JJV"
  特徴: slate-gray
  状態: 壊れている: X-4832-TPL を欠いた T-9247-OME, F-0010-DVZ・J-1623-IYI を欠いた B-1403-YSS が足りない
H-6458-ZDH: "an exemplary instance of part number H-6458-ZDH"
  特徴: camouflage-green
  状態: 壊れている: X-4832-TPL を欠いた T-9247-OME・F-0010-DVZ・J-1623-IYI を欠いた B-1403-YSS を欠いた P-6678-JJV, L-5065-EGO が足りない
L-5065-EGO: "an exemplary instance of part number L-5065-EGO"
  特徴: gray20
  状態: 無傷
L-5065-EGO: "an exemplary instance of part number L-5065-EGO"
  特徴: pale-green
  状態: 無傷
T-6678-BOP: "an exemplary instance of part number T-6678-BOP"
  特徴: amethyst
  状態: 壊れている: D-6458-HIP, X-9247-GRW, H-5065-MLW, L-6678-ROF, F-5065-QBC が足りない
L-6678-ROF: "an exemplary instance of part number L-6678-ROF"
  特徴: pink
  状態: 壊れている: L-6678-ROF, D-6458-HIP が足りない
P-9247-WRM: "an exemplary instance of part number P-9247-WRM"
  特徴: moccasin
  状態: 壊れている: T-4832-BUT, X-1403-GXC が足りない
D-6458-HIP: "an exemplary instance of part number D-6458-HIP"
  特徴: cornflower-blue
  状態: 壊れている: B-0010-LBJ が足りない
L-6678-ROF: "an exemplary instance of part number L-6678-ROF"
  特徴: organic
  状態: 壊れている: F-1623-QEQ が足りない
F-1623-QEQ: "an exemplary instance of part number F-1623-QEQ"
  特徴: fuchsia
  状態: 壊れている: J-4292-VHX が足りない
J-4292-VHX: "an exemplary instance of part number J-4292-VHX"
  特徴: lavender
  状態: 壊れている: V-5065-KQU を欠いた N-9887-AKG が足りない
N-9887-AKG: "an exemplary instance of part number N-9887-AKG"
  特徴: lemon-cream
  状態: 壊れている: V-5065-KQU が足りず、それを補ってもなお R-6458-FNN を欠く
Z-6678-PTD: "an exemplary instance of part number Z-6678-PTD"
  特徴: reverse-chirality
  状態: 壊れている: L-1403-EDY が足りず、それを補ってもなお D-9247-UWK, H-4832-ZAR を欠く
B-0010-LBJ: "an exemplary instance of part number B-0010-LBJ"
  特徴: lime
  状態: 壊れている: V-5065-KQU を欠いた N-9887-AKG が足りない
N-9887-AKG: "an exemplary instance of part number N-9887-AKG"
  特徴: sapphire
  状態: 壊れている: V-5065-KQU が足りず、それを補ってもなお R-6458-FNN を欠く
H-5065-MLW: "an exemplary instance of part number H-5065-MLW"
  特徴: blanched-almond-colored
  状態: 無傷
D-6458-HIP: "an exemplary instance of part number D-6458-HIP"
  特徴: gray20
  状態: 壊れている: X-4292-TMV を欠いた T-1623-OJO・B-9887-YPE を欠いた P-0010-JGH, J-5065-IVS が足りない
N-6678-NYZ: "an exemplary instance of part number N-6678-NYZ"
  特徴: water-logged
  状態: 無傷
R-9247-SCI: "an exemplary instance of part number R-9247-SCI"
  特徴: persian-blue
  状態: 無傷
J-5065-IVS: "an exemplary instance of part number J-5065-IVS"
  特徴: celadon
  状態: 壊れている: R-9247-SCI, N-6678-NYZ, L-6678-ROF が足りない
V-4832-XFP: "an exemplary instance of part number V-4832-XFP"
  特徴: magenta
  状態: 無傷
P-9887-WUC: "an exemplary instance of part number P-9887-WUC"
  特徴: momemtum-preserving
  状態: 無傷
Z-1403-CIW: "an exemplary instance of part number Z-1403-CIW"
  特徴: bright-green
  状態: 壊れている: D-0010-HLF, L-4292-RRT を欠いた H-1623-MOM, F-6458-DSL が足りない
F-6458-DSL: "an exemplary instance of part number F-6458-DSL"
  特徴: school-bus-yellow
  状態: 壊れている: D-6458-HIP が足りない
T-6458-BXJ: "an exemplary instance of part number T-6458-BXJ"
  特徴: rosy-brown
  状態: 壊れている: F-9247-QHG, J-4832-VKN が足りず、それらを補ってもなお X-5065-GBQ, B-6678-LEX を欠く
F-6458-DSL: "an exemplary instance of part number F-6458-DSL"
  特徴: pale-magenta
  状態: 壊れている: F-9247-QHG・J-4832-VKN を欠いた T-6458-BXJ が足りない
P-0010-JGH: "an exemplary instance of part number P-0010-JGH"
  特徴: tan
  状態: 壊れている: B-9887-YPE, D-0010-HLF・L-4292-RRT を欠いた H-1623-MOM を欠いた Z-1403-CIW, D-0010-HLF・L-4292-RRT を欠いた H-1623-MOM を欠いた Z-1403-CIW, N-1403-ANU, V-4832-XFP, X-4292-TMV を欠いた T-1623-OJO が足りない
N-1403-ANU: "an exemplary instance of part number N-1403-ANU"
  特徴: pale-pink
  状態: 無傷
Z-1403-CIW: "an exemplary instance of part number Z-1403-CIW"
  特徴: goldenrod
  状態: 壊れている: D-0010-HLF, L-4292-RRT を欠いた H-1623-MOM が足りない
D-6458-HIP: "an exemplary instance of part number D-6458-HIP"
  特徴: pear
  状態: 無傷
N-9887-AKG: "an exemplary instance of part number N-9887-AKG"
  特徴: dodger-blue
  状態: 壊れている: V-5065-KQU が足りず、それを補ってもなお R-6458-FNN を欠く
P-5065-WLI: "an exemplary instance of part number P-5065-WLI"
  特徴: celadon
  状態: 無傷
D-4292-HCL: "an exemplary instance of part number D-4292-HCL"
  特徴: discounted
  状態: 壊れている: H-9887-MFS が足りない
B-5065-YGK: "an exemplary instance of part number B-5065-YGK"
  特徴: indigo
  状態: 壊れている: R-1403-SSO, Z-1623-CYE を欠いた V-0010-XVV が足りず、それらを補ってもなお J-9247-IMY, N-4832-NPH を欠き、さらに F-6678-DJR を欠く
jumper shunt: "隣り合うピンに砂を流し込むための、ごく普通のシャント"
  特徴: なし
  状態: 壊れている: H-9887-MFS を欠いた D-4292-HCL, X-9247-GRW を欠いた T-6678-BOP, R-1403-SSO・Z-1623-CYE を欠いた V-0010-XVV を欠いた B-5065-YGK, L-6458-RIZ が足りない
B-4832-LUF: "an exemplary instance of part number B-4832-LUF"
  特徴: peach-yellow
  状態: 壊れている: J-0010-VBT・N-1623-AEC を欠いた F-1403-QXM が足りない
R-4292-FHJ: "an exemplary instance of part number R-4292-FHJ"
  特徴: low-carb
  状態: 壊れている: V-9887-KKQ が足りない
T-6678-BOP: "an exemplary instance of part number T-6678-BOP"
  特徴: mauve
  状態: 壊れている: X-9247-GRW が足りない
P-4832-JAD: "an exemplary instance of part number P-4832-JAD"
  特徴: dodger-blue
  状態: 無傷
L-6458-RIZ: "an exemplary instance of part number L-6458-RIZ"
  特徴: gray90
  状態: 壊れている: T-1403-ODK, H-6678-ZTN・L-9247-EWU を欠いた Z-6458-PNX が足りない
X-0010-TGR: "an exemplary instance of part number X-0010-TGR"
  特徴: gargantuan
  状態: 壊れている: Z-9247-CCS が足りず、それを補ってもなお R-5065-SVE, V-6678-XYL を欠き、さらに B-1623-YJY, J-9887-IPO・N-6458-NSV を欠いた F-4292-DMH を欠く
P-4832-JAD: "an exemplary instance of part number P-4832-JAD"
  特徴: gamboge
  状態: 無傷
T-1403-ODK: "an exemplary instance of part number T-1403-ODK"
  特徴: blue
  状態: 無傷
D-4292-HCL: "an exemplary instance of part number D-4292-HCL"
  特徴: water-logged
  状態: 壊れている: H-9887-MFS が足りない
Z-6458-PNX: "an exemplary instance of part number Z-6458-PNX"
  特徴: blue
  状態: 壊れている: H-6678-ZTN, L-9247-EWU, H-1403-MII・L-0010-RLP を欠いた D-4832-HFZ が足りず、それらを補ってもなお D-5065-UQG を欠く
T-4292-BRF: "an exemplary instance of part number T-4292-BRF"
  特徴: pale-violet-red
  状態: 無傷
D-4832-HFZ: "an exemplary instance of part number D-4832-HFZ"
  特徴: spring-green
  状態: 壊れている: H-1403-MII, L-0010-RLP, T-4292-BRF が足りない
D-4292-HCL: "an exemplary instance of part number D-4292-HCL"
  特徴: sapphire
  状態: 壊れている: H-9887-MFS, H-1403-MII・L-0010-RLP を欠いた D-4832-HFZ が足りない
D-4292-HCL: "an exemplary instance of part number D-4292-HCL"
  特徴: jade
  状態: 壊れている: P-4832-JAD, H-9887-MFS が足りない
P-4832-JAD: "an exemplary instance of part number P-4832-JAD"
  特徴: cyan
  状態: 壊れている: B-6458-LXT を欠いた X-9887-GUM が足りない
D-4832-HFZ: "an exemplary instance of part number D-4832-HFZ"
  特徴: flax
  状態: 壊れている: H-1403-MII, L-0010-RLP が足りない
F-5065-QBC: "an exemplary instance of part number F-5065-QBC"
  特徴: periwinkle
  状態: 無傷
X-9887-GUM: "an exemplary instance of part number X-9887-GUM"
  特徴: blaze-orange
  状態: 壊れている: J-6678-VEJ, X-9247-GRW を欠いた T-6678-BOP, N-9247-AHQ, B-6458-LXT が足りない
N-9247-AHQ: "an exemplary instance of part number N-9247-AHQ"
  特徴: lemon-cream
  状態: 無傷
P-1623-WOW: "an exemplary instance of part number P-1623-WOW"
  特徴: viridian
  状態: 無傷
D-1623-UTU: "an exemplary instance of part number D-1623-UTU"
  特徴: gray70
  状態: 無傷
R-4832-FKX: "an exemplary instance of part number R-4832-FKX"
  特徴: snow
  状態: 壊れている: V-1403-KNG, T-5065-OGY を欠いた H-4292-ZWD, Z-0010-PQN が足りない
H-4292-ZWD: "an exemplary instance of part number H-4292-ZWD"
  特徴: hypoallergenic
  状態: 壊れている: H-1403-MII・L-0010-RLP を欠いた D-4832-HFZ, T-5065-OGY が足りず、それらを補ってもなお L-9887-EAK, P-6458-JDR を欠く
D-4832-HFZ: "an exemplary instance of part number D-4832-HFZ"
  特徴: gargantuan
  状態: 壊れている: H-1403-MII, L-0010-RLP, H-6678-ZTN・L-9247-EWU を欠いた Z-6458-PNX が足りない
Z-6458-PNX: "an exemplary instance of part number Z-6458-PNX"
  特徴: shiny
  状態: 壊れている: H-6678-ZTN, L-9247-EWU が足りず、それらを補ってもなお D-5065-UQG を欠く
B-5065-YGK: "an exemplary instance of part number B-5065-YGK"
  特徴: rust
  状態: 壊れている: R-1403-SSO, B-9247-YMO, Z-1623-CYE を欠いた V-0010-XVV が足りず、それらを補ってもなお J-9247-IMY, N-4832-NPH を欠き、さらに F-6678-DJR を欠く
X-6678-TJH: "an exemplary instance of part number X-6678-TJH"
  特徴: gray90
  状態: 壊れている: F-4832-DPV が足りない
F-4832-DPV: "an exemplary instance of part number F-4832-DPV"
  特徴: blanched-almond-colored
  状態: 無傷
N-0010-NVL: "an exemplary instance of part number N-0010-NVL"
  特徴: sangria
  状態: 無傷
B-9247-YMO: "an exemplary instance of part number B-9247-YMO"
  特徴: burgundy
  状態: 壊れている: J-1403-ISE, R-1623-SYS が足りない
R-1623-SYS: "an exemplary instance of part number R-1623-SYS"
  特徴: fuchsia
  状態: 壊れている: V-4292-XCZ が足りない
V-4292-XCZ: "an exemplary instance of part number V-4292-XCZ"
  特徴: plum
  状態: 無傷
J-1403-ISE: "an exemplary instance of part number J-1403-ISE"
  特徴: burgundy
  状態: 無傷
J-6678-VEJ: "an exemplary instance of part number J-6678-VEJ"
  特徴: green
  状態: 壊れている: V-1403-KNG・Z-0010-PQN を欠いた R-4832-FKX が足りない
P-5065-WLI: "an exemplary instance of part number P-5065-WLI"
  特徴: gray80
  状態: 無傷

--- 交差点: 54番地／ドーチェスター・アベニュー ---
54番地とドーチェスター・アベニューの交差点に立っている。ここから北、東へ進むことができる。

T-9887-OAU: "an exemplary instance of part number T-9887-OAU"
  特徴: terra-cotta
  状態: 壊れている: H-6458-ZNJ が足りず、それを補ってもなお L-1623-ETG を欠く
L-1623-ETG: "an exemplary instance of part number L-1623-ETG"
  特徴: carrot
  状態: 無傷
B-1403-YDU: "an exemplary instance of part number B-1403-YDU"
  特徴: heliotrope
  状態: 壊れている: N-5065-ABM を欠いた L-4832-RFL, F-0010-DGD が足りない
H-9247-MCE: "an exemplary instance of part number H-9247-MCE"
  特徴: imported
  状態: 壊れている: P-6678-JTX が足りない
R-0010-FBF: "an exemplary instance of part number R-0010-FBF"
  特徴: pear
  状態: 無傷
Z-4832-PKJ: "an exemplary instance of part number Z-4832-PKJ"
  特徴: pale-turquoise
  状態: 壊れている: D-1403-UNQ が足りない
T-0010-BLZ: "an exemplary instance of part number T-0010-BLZ"
  特徴: sandy-brown
  状態: 壊れている: D-1403-UNQ を欠いた Z-4832-PKJ, P-6678-JTX を欠いた H-9247-MCE が足りない
R-6678-FET: "an exemplary instance of part number R-6678-FET"
  特徴: miniature
  状態: 壊れている: V-9247-KHC が足りない
H-9247-MCE: "an exemplary instance of part number H-9247-MCE"
  特徴: linen-colored
  状態: 壊れている: V-9247-KHC を欠いた R-6678-FET, P-6678-JTX, T-0010-BLZ が足りない
L-4832-RFL: "an exemplary instance of part number L-4832-RFL"
  特徴: snow
  状態: 壊れている: N-5065-ABM が足りず、それを補ってもなお J-6458-VXF を欠く
H-0010-ZQX: "an exemplary instance of part number H-0010-ZQX"
  特徴: lilac
  状態: 壊れている: L-1623-ETG が足りない
X-1623-GOI: "an exemplary instance of part number X-1623-GOI"
  特徴: gray30
  状態: 壊れている: F-9887-QUW が足りず、それを補ってもなお B-4292-LRP を欠く
L-4832-RFL: "an exemplary instance of part number L-4832-RFL"
  特徴: silver
  状態: 壊れている: P-1403-WIS が足りない
V-1623-KEM: "an exemplary instance of part number V-1623-KEM"
  特徴: bisque
  状態: 壊れている: P-6678-JTX を欠いた H-9247-MCE, P-1403-WIS を欠いた L-4832-RFL が足りない
N-4013-DJW: "an exemplary instance of part number N-4013-DJW"
  特徴: gray40
  状態: 壊れている: L-1623-ETG を欠いた H-0010-ZQX が足りない
T-9247-OWG: "an exemplary instance of part number T-9247-OWG"
  特徴: pale-red-violet
  状態: 壊れている: F-0010-DGD を欠いた B-1403-YDU が足りない
F-0010-DGD: "an exemplary instance of part number F-0010-DGD"
  特徴: peach
  状態: 壊れている: T-9247-OWG が足りない
H-6458-ZNJ: "an exemplary instance of part number H-6458-ZNJ"
  特徴: pine-green
  状態: 無傷
X-4832-TAN: "an exemplary instance of part number X-4832-TAN"
  特徴: persian-blue
  状態: 無傷
P-4292-JWN: "an exemplary instance of part number P-4292-JWN"
  特徴: pink
  状態: 壊れている: B-1403-YDU を欠いた T-9247-OWG が足りない
F-0010-DGD: "an exemplary instance of part number F-0010-DGD"
  特徴: coral
  状態: 壊れている: D-6678-HYV が足りず、それを補ってもなお Z-5065-CVO を欠き、さらに Z-5065-CVO を欠く
Z-4292-PHT: "an exemplary instance of part number Z-4292-PHT"
  特徴: salmon-colored
  状態: 壊れている: B-1403-YDU, D-6678-HYV を欠いた F-0010-DGD が足りず、それらを補ってもなお V-6458-XSH を欠いた F-0010-DGD を欠き、さらに R-9887-SPY を欠く
B-1403-YDU: "an exemplary instance of part number B-1403-YDU"
  特徴: teal
  状態: 壊れている: N-4292-NMR を欠いた J-1623-IJK, B-1403-YDU を欠いた Z-4292-PHT, F-0010-DGD が足りない
J-1623-IJK: "an exemplary instance of part number J-1623-IJK"
  特徴: old-gold
  状態: 壊れている: N-4292-NMR が足りない
X-4832-TAN: "an exemplary instance of part number X-4832-TAN"
  特徴: ghost-white
  状態: 無傷
P-6678-JTX: "an exemplary instance of part number P-6678-JTX"
  特徴: imitation
  状態: 壊れている: X-4832-TAN が足りない
Z-4292-PHT: "an exemplary instance of part number Z-4292-PHT"
  特徴: peach-orange
  状態: 壊れている: P-6678-JTX, T-9247-OWG, L-5065-EQQ, T-9887-OAU が足りない
N-4013-DJW: "an exemplary instance of part number N-4013-DJW"
  特徴: yellow
  状態: 壊れている: R-0010-FBF, P-4292-JWN が足りない
Z-4292-PHT: "an exemplary instance of part number Z-4292-PHT"
  特徴: red-violet
  状態: 壊れている: L-5065-EQQ, L-5065-EQQ を欠いた Z-4292-PHT が足りない
H-6458-ZNJ: "an exemplary instance of part number H-6458-ZNJ"
  特徴: purple
  状態: 無傷
power cord: "屋内・屋外で使用できる"
  特徴: なし
  状態: 壊れている: N-4013-DJW, V-1623-KEM, H-6458-ZNJ, L-5065-EQQ を欠いた Z-4292-PHT が足りない
Z-4292-PHT: "an exemplary instance of part number Z-4292-PHT"
  特徴: purple
  状態: 壊れている: L-5065-EQQ が足りない
H-6458-ZNJ: "an exemplary instance of part number H-6458-ZNJ"
  特徴: gray90
  状態: 壊れている: L-5065-EQQ を欠いた Z-4292-PHT が足りない
Z-4292-PHT: "an exemplary instance of part number Z-4292-PHT"
  特徴: violet
  状態: 壊れている: D-9887-UKC が足りない
```
