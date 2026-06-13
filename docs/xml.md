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
