# Loading Bar

<MessageBar layout=MessageBarLayout::Multiline intent=MessageBarIntent::Warning>
    <MessageBarBody>
      <h3 style="margin: 0">"Prerequisite"</h3>
      <p>
        "If you want to use loading bar, you need to wrap the component where you call related methods inside LoadingBarProvider and use use_loading_bar to get the API."
      </p>
    </MessageBarBody>
</MessageBar>

```rust demo
let loading_bar = LoadingBarInjection::expect_use();
let start = move |_| {
    loading_bar.start();
};
let finish = move |_| {
    loading_bar.finish();
};
let error = move |_| {
    loading_bar.error();
};

view! {
    <Space>
        <Button on_click=start>"start"</Button>
        <Button on_click=finish>"finish"</Button>
        <Button on_click=error>"error"</Button>
    </Space>
}
```

### LoadingBarProvider Injection Methods

| Name   | Type        | Description                                                  |
| ------ | ----------- | ------------------------------------------------------------ |
| start  | `fn(&self)` | Callback function for loading bar to start loading.          |
| finish | `fn(&self)` | The callback function when the loading bar finishes loading. |
| error  | `fn(&self)` | Callback function for loading bar error.                     |
