# lesson-3

## 修改代码Kitties.js代码
```js
import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  // 使用hooks，初始化
  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
    // 封装异步调用
    const kittiesInfo = async () => {
      const result = await api.query.kittiesModule.kittiesCount()
      console.log(result)
      if (result.isEmpty === false) {
        // 获取count数据
        const count = result.value.words[0]
        console.log(count)
        // 构造迭代数组
        const idList = [...Array(count).keys()]
        console.log(idList)
        // 批量获取dna
        const dna = await api.query.kittiesModule.kitties.multi(idList)
        console.log(dna)
        // 批量获取owner
        const owner = await api.query.kittiesModule.owner.multi(idList)
        console.log(owner)
        const kitties = []
        // 插入小猫
        for (let i = 0; i < count; i++) {
          const kitty = {}
          kitty.id = i
          kitty.dna = dna[i].value
          kitty.owner = owner[i].value.toString()
          kitties.push(kitty)
        }
        // 更新小猫数据
        setKitties(kitties)
      }
    }
    kittiesInfo()
  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    const kittiesInfo = async () => {
      const result = await api.query.kittiesModule.kittiesCount()
      console.log(result)
      if (result.isEmpty === false) {
        const count = result.value.words[0]
        console.log(count)
        const idList = [...Array(count).keys()]
        console.log(idList)
        const dna = await api.query.kittiesModule.kitties.multi(idList)
        console.log(dna)
        const owner = await api.query.kittiesModule.owner.multi(idList)
        console.log(owner)
        const kitties = []
        for (let i = 0; i < count; i++) {
          const kitty = {}
          kitty.id = i
          kitty.dna = dna[i].value
          kitty.owner = owner[i].value.toString()
          kitties.push(kitty)
        }
        setKitties(kitties)
      }
    }
    kittiesInfo()
  }

  useEffect(fetchKitties, [api, keyring, status])
  // 首次加载页面触发
  useEffect(populateKitties, [])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}

```

### 运行截图

* 初始化

![1.png](https://github.com/zongxunjie/SA/blob/main/lesson-3/1.png)

* Alice创建小毛孩

![2.png](https://github.com/zongxunjie/SA/blob/main/lesson-3/2.png)

* Bob创建小毛孩

![3.png](https://github.com/zongxunjie/SA/blob/main/lesson-3/3.png)

* Bob转移小毛孩给Alice

![4.png](https://github.com/zongxunjie/SA/blob/main/lesson-3/4.png)

* Alice查看小毛孩

![5.png](https://github.com/zongxunjie/SA/blob/main/lesson-3/5.png)

* Bob查看小毛孩并创建小毛孩

![6.png](https://github.com/zongxunjie/SA/blob/main/lesson-3/6.png)
