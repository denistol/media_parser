# Media Parser

**Media Parser** is a project for parsing news from popular Ukrainian news websites. It extracts news headlines and their links using CSS selectors to navigate the web page structure.

## Features

- News scraping from multiple Ukrainian sources.
- Easy configuration using CSS selectors for element extraction.
- Support for popular Ukrainian news sites such as `5 Kanal`, `Ukrainska Pravda`, `TSN`, `RBK-Ukraine`, and others.
- Suitable for further data processing or integration into other projects.

## Supported Sources

The project supports the following news websites:

- [5 Kanal](https://www.5.ua/)
- [Ukrainska Pravda](https://www.pravda.com.ua/)
- [TSN](https://tsn.ua/)
- [RBK-Ukraine](https://www.rbc.ua/ukr)
- [BBC Ukraine](https://www.bbc.com/ukrainian)
- [LIGA.net](https://news.liga.net/ua)
- [Interfax-Ukraine](https://interfax.com.ua/)
- [Ukraina 24](https://24tv.ua/)
- [Glavcom](https://glavcom.ua/)
- [NewsOne](https://1newsone.com.ua/)
- [UNIAN](https://www.unian.ua/detail/all_news)
- [Fakty](https://fakty.ua/)
- [Espreso](https://espreso.tv/news)
- [112 Ukraine](https://112.ua/)
- [Gazeta.ua](https://gazeta.ua/)
- [Pramy TV Channel](https://prm.ua/)

### Example Object for Scraping:

```json
{
    "url": "https://www.5.ua/",
    "name": "5 Kanal",
    "container_selector": ".nf-latestnews",
    "item_selector": ".nf-latestnews-post",
    "title_selector": ".nf-latestnews-post-lnk"
}
