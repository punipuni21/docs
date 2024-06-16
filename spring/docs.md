SpringFramework 5.0でRestTemplateがメンテナンスモードに入っている

メンテナンスモード：バグや脆弱性対応のみ受け付ける(まだ非推奨ではない)
```
	As of 5.0 the RestTemplate is in maintenance mode, with only minor requests for changes and bugs to be accepted going forward. Please, consider using the WebClient which offers a more modern API and supports sync, async, and streaming scenarios.
```

詳細：[5.2.23.RELEASE](https://spring.pleiades.io/spring-framework/docs/5.2.23.RELEASE/spring-framework-reference/web.html#webmvc-resttemplate)

[6.1.9](https://docs.spring.io/spring-framework/reference/web/webmvc-client.html)のドキュメントではメンテナンスの記載がない．(5系リリース時)に周知してあるからか？

[RestTemplate to WebClient](https://spring.pleiades.io/spring-framework/reference/integration/rest-clients.html#_migrating_from_resttemplate_to_restclient)

## docs
[リアクティブなHTTPクライアント! WebClient入門](https://news.mynavi.jp/techplus/article/techp5348/)

[Spring Framework 6.1 から追加された RestClient を試してみる](https://qiita.com/comware_tamura/items/35901c66579f2f8563e9)

https://spring.pleiades.io/spring-framework/reference/integration/rest-clients.html#rest-webclient