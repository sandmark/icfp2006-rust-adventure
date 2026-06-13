# XML レスポンス

## 概要
`adventure` は以下のようなものだ。

- クエリ言語はコマンド (e.g., `combine foo with bar`)
- 結果を XML で出力する思考エンジン and/or データベース

`switch english` (default) では断片的な情報しか得られず、
アイテムや周囲の情景などは `examine` で調べ回ることになる。

一方で `switch xml` では、スタック (地面) に積んである
アイテムの詳細などがすべて一度に取得できる。

ここではその契約を記録する。

## スタート地点 (Room With a Door)

    Room With a Door
    
    You are in a room with a mechanical door. You will probably need
    to use a keypad to unlock it. A hallway leads north.
    There is a pamphlet here.
    Underneath the pamphlet, there is a manifesto.

`./adventure` 起動後は `English` で描画される。

`switch xml` すると、以下のレスポンスが返ってくる。

```xml
<success>
  <command>
    <switch>
      XML
    </switch>
  </command>
</success>
```

`examine` すると、元々のデータは構造化されていることがわかる。
また ground stack に積まれたアイテムの description もすべて記載される。

NOTE: `examine` は `look` のエイリアスであることもわかる。

`description` は必ずしも `String` ではなく、
`<redacted/>` であるかもしれないことがここで示されている。

```xml
<success>
  <command>
    <look>
      <room>
        <name>
          Room With a Door
        </name>
        <description>
          You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north.
        </description>
        <items>
          <item>
            <name>
              pamphlet
            </name>
            <description>
              standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of "One Resident's Trash Is Another Resident's Treasure," unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day
            </description>
            <adjectives>
            </adjectives>
            <condition>
              <pristine>
              </pristine>
            </condition>
            <piled_on>
              <item>
                <name>
                  manifesto
                </name>
                <description>
                  <redacted/>
                </description>
                <adjectives>
                </adjectives>
                <condition>
                  <pristine>
                  </pristine>
                </condition>
                <piled_on>
                </piled_on>
              </item>
            </piled_on>
          </item>
        </items>
      </room>
    </look>
  </command>
</success>
```

### help コマンド

この状態でもクエリ (user input) は変わらない。

`help examine` してみると以下が返る。

```xml
<help>
  examine: Inspect an item or your environment. Synonyms include ex, x, look, and l.
</help>
```

## error タグ

### when command not found
存在しないコマンド `foobarbaz` のレスポンスは以下のようになる。

```xml
<error>
  <response>
    Huh? Try 'help'.
  </response>
</error>
```

### when combine failed
TODO: あとで調べる。

## command log

- `take pamphlet`

```xml
<success>
  <command>
    <take>
      <item>
        <name>
          pamphlet
        </name>
        <description>
          standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of "One Resident's Trash Is Another Resident's Treasure," unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day
        </description>
        <adjectives>
        </adjectives>
        <condition>
          <pristine>
          </pristine>
        </condition>
        <piled_on>
        </piled_on>
      </item>
    </take>
  </command>
</success>
```

- `inc pamphlet`

**XML モードでもダミー報酬コードが混じるケースがある。エッジケースを考慮すること。**

```xml
ADVTR.INC=5@999999|f95731ab88952dfa4cb326fb99c085f
<success>
  <command>
    <incinerate>
      <item>
        <name>
          pamphlet
        </name>
        <description>
          standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of "One Resident's Trash Is Another Resident's Treasure," unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day
        </description>
        <adjectives>
        </adjectives>
        <condition>
          <pristine>
          </pristine>
        </condition>
        <piled_on>
        </piled_on>
      </item>
    </incinerate>
  </command>
</success>
```

- (manifesto を拾っていない状態で) `inc manifesto`

```xml
<error>
  <response>
    You aren't carrying a manifesto. You can only incinerate items in your possession.
  </response>
</error>
```

- `take manifesto`
```xml
<success>
  <command>
    <take>
      <item>
        <name>
          manifesto
        </name>
        <description>
          <redacted/>
        </description>
        <adjectives>
        </adjectives>
        <condition>
          <pristine>
          </pristine>
        </condition>
        <piled_on>
        </piled_on>
      </item>
    </take>
  </command>
</success>
```

- `inc manifesto`
```xml
<success>
  <command>
    <incinerate>
      <item>
        <name>
          manifesto
        </name>
        <description>
          <redacted/>
        </description>
        <adjectives>
        </adjectives>
        <condition>
          <pristine>
          </pristine>
        </condition>
        <piled_on>
        </piled_on>
      </item>
    </incinerate>
  </command>
</success>
```

- `examine`

```xml
<success>
  <command>
    <look>
      <room>
        <name>
          Room With a Door
        </name>
        <description>
          You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north.
        </description>
        <items>
        </items>
      </room>
    </look>
  </command>
</success>
```

- `go north` (with implicit `examine`)

```xml
<success>
  <command>
    <go>
      <room>
        <name>
          Junk Room
        </name>
        <description>
          You are in a room with a pile of junk. A hallway leads south.
        </description>
        <items>
          <item>
            <name>
              bolt
            </name>
            <description>
              quite useful for securing all sorts of things
            </description>
            <adjectives>
            </adjectives>
            <condition>
              <pristine>
              </pristine>
            </condition>
            <piled_on>
              <item>
                <name>
                  spring
                </name>
                <description>
                  tightly coiled
                </description>
                <adjectives>
                </adjectives>
                <condition>
                  <pristine>
                  </pristine>
                </condition>
                <piled_on>
                  <item>
                    <name>
                      button
                    </name>
                    <description>
                      labeled 6
                    </description>
                    <adjectives>
                    </adjectives>
                    <condition>
                      <pristine>
                      </pristine>
                    </condition>
                    <piled_on>
                      <item>
                        <name>
                          processor
                        </name>
                        <description>
                          from the elusive 19x86 line
                        </description>
                        <adjectives>
                        </adjectives>
                        <condition>
                          <broken>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                            <missing>
                              <kind>
                                <name>
                                  cache
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                            </missing>
                          </broken>
                        </condition>
                        <piled_on>
                          <item>
                            <name>
                              pill
                            </name>
                            <description>
                              tempting looking
                            </description>
                            <adjectives>
                              <adjective>
                                red
                              </adjective>
                            </adjectives>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                            <piled_on>
                              <item>
                                <name>
                                  radio
                                </name>
                                <description>
                                  a hi-fi AM/FM stereophonic radio
                                </description>
                                <adjectives>
                                </adjectives>
                                <condition>
                                  <broken>
                                    <condition>
                                      <pristine>
                                      </pristine>
                                    </condition>
                                    <missing>
                                      <kind>
                                        <name>
                                          transistor
                                        </name>
                                        <condition>
                                          <pristine>
                                          </pristine>
                                        </condition>
                                      </kind>
                                      <kind>
                                        <name>
                                          antenna
                                        </name>
                                        <condition>
                                          <pristine>
                                          </pristine>
                                        </condition>
                                      </kind>
                                    </missing>
                                  </broken>
                                </condition>
                                <piled_on>
                                  <item>
                                    <name>
                                      cache
                                    </name>
                                    <description>
                                      fully-associative
                                    </description>
                                    <adjectives>
                                    </adjectives>
                                    <condition>
                                      <pristine>
                                      </pristine>
                                    </condition>
                                    <piled_on>
                                      <item>
                                        <name>
                                          transistor
                                        </name>
                                        <description>
                                          PNP-complete
                                        </description>
                                        <adjectives>
                                          <adjective>
                                            blue
                                          </adjective>
                                        </adjectives>
                                        <condition>
                                          <pristine>
                                          </pristine>
                                        </condition>
                                        <piled_on>
                                          <item>
                                            <name>
                                              antenna
                                            </name>
                                            <description>
                                              appropriate for receiving transmissions between 30 kHz and 30 MHz
                                            </description>
                                            <adjectives>
                                            </adjectives>
                                            <condition>
                                              <pristine>
                                              </pristine>
                                            </condition>
                                            <piled_on>
                                              <item>
                                                <name>
                                                  screw
                                                </name>
                                                <description>
                                                  not from a Dutch company
                                                </description>
                                                <adjectives>
                                                </adjectives>
                                                <condition>
                                                  <pristine>
                                                  </pristine>
                                                </condition>
                                                <piled_on>
                                                  <item>
                                                    <name>
                                                      motherboard
                                                    </name>
                                                    <description>
                                                      well-used
                                                    </description>
                                                    <adjectives>
                                                    </adjectives>
                                                    <condition>
                                                      <broken>
                                                        <condition>
                                                          <pristine>
                                                          </pristine>
                                                        </condition>
                                                        <missing>
                                                          <kind>
                                                            <name>
                                                              A-1920-IXB
                                                            </name>
                                                            <condition>
                                                              <pristine>
                                                              </pristine>
                                                            </condition>
                                                          </kind>
                                                          <kind>
                                                            <name>
                                                              screw
                                                            </name>
                                                            <condition>
                                                              <pristine>
                                                              </pristine>
                                                            </condition>
                                                          </kind>
                                                        </missing>
                                                      </broken>
                                                    </condition>
                                                    <piled_on>
                                                      <item>
                                                        <name>
                                                          A-1920-IXB
                                                        </name>
                                                        <description>
                                                          an exemplary instance of part number A-1920-IXB
                                                        </description>
                                                        <adjectives>
                                                        </adjectives>
                                                        <condition>
                                                          <broken>
                                                            <condition>
                                                              <broken>
                                                                <condition>
                                                                  <pristine>
                                                                  </pristine>
                                                                </condition>
                                                                <missing>
                                                                  <kind>
                                                                    <name>
                                                                      transistor
                                                                    </name>
                                                                    <condition>
                                                                      <pristine>
                                                                      </pristine>
                                                                    </condition>
                                                                  </kind>
                                                                </missing>
                                                              </broken>
                                                            </condition>
                                                            <missing>
                                                              <kind>
                                                                <name>
                                                                  radio
                                                                </name>
                                                                <condition>
                                                                  <broken>
                                                                    <condition>
                                                                      <pristine>
                                                                      </pristine>
                                                                    </condition>
                                                                    <missing>
                                                                      <kind>
                                                                        <name>
                                                                          antenna
                                                                        </name>
                                                                        <condition>
                                                                          <pristine>
                                                                          </pristine>
                                                                        </condition>
                                                                      </kind>
                                                                    </missing>
                                                                  </broken>
                                                                </condition>
                                                              </kind>
                                                              <kind>
                                                                <name>
                                                                  processor
                                                                </name>
                                                                <condition>
                                                                  <pristine>
                                                                  </pristine>
                                                                </condition>
                                                              </kind>
                                                              <kind>
                                                                <name>
                                                                  bolt
                                                                </name>
                                                                <condition>
                                                                  <pristine>
                                                                  </pristine>
                                                                </condition>
                                                              </kind>
                                                            </missing>
                                                          </broken>
                                                        </condition>
                                                        <piled_on>
                                                          <item>
                                                            <name>
                                                              transistor
                                                            </name>
                                                            <description>
                                                              NPN-complete
                                                            </description>
                                                            <adjectives>
                                                              <adjective>
                                                                red
                                                              </adjective>
                                                            </adjectives>
                                                            <condition>
                                                              <pristine>
                                                              </pristine>
                                                            </condition>
                                                            <piled_on>
                                                              <item>
                                                                <name>
                                                                  keypad
                                                                </name>
                                                                <description>
                                                                  labeled "use me"
                                                                </description>
                                                                <adjectives>
                                                                </adjectives>
                                                                <condition>
                                                                  <broken>
                                                                    <condition>
                                                                      <pristine>
                                                                      </pristine>
                                                                    </condition>
                                                                    <missing>
                                                                      <kind>
                                                                        <name>
                                                                          motherboard
                                                                        </name>
                                                                        <condition>
                                                                          <pristine>
                                                                          </pristine>
                                                                        </condition>
                                                                      </kind>
                                                                      <kind>
                                                                        <name>
                                                                          button
                                                                        </name>
                                                                        <condition>
                                                                          <pristine>
                                                                          </pristine>
                                                                        </condition>
                                                                      </kind>
                                                                    </missing>
                                                                  </broken>
                                                                </condition>
                                                                <piled_on>
                                                                  <item>
                                                                    <name>
                                                                      trash
                                                                    </name>
                                                                    <description>
                                                                      of absolutely no value
                                                                    </description>
                                                                    <adjectives>
                                                                    </adjectives>
                                                                    <condition>
                                                                      <pristine>
                                                                      </pristine>
                                                                    </condition>
                                                                    <piled_on>
                                                                    </piled_on>
                                                                  </item>
                                                                </piled_on>
                                                              </item>
                                                            </piled_on>
                                                          </item>
                                                        </piled_on>
                                                      </item>
                                                    </piled_on>
                                                  </item>
                                                </piled_on>
                                              </item>
                                            </piled_on>
                                          </item>
                                        </piled_on>
                                      </item>
                                    </piled_on>
                                  </item>
                                </piled_on>
                              </item>
                            </piled_on>
                          </item>
                        </piled_on>
                      </item>
                    </piled_on>
                  </item>
                </piled_on>
              </item>
            </piled_on>
          </item>
        </items>
      </room>
    </go>
  </command>
</success>
```
