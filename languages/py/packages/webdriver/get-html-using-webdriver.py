#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
# !require selenium packge!
# if not installed then run following shell command
# pip install selenium
from selenium.webdriver import Chrome, ChromeOptions


options = ChromeOptions()
options.add_argument('--headless')
driver = Chrome(options=options)


driver.get('https://gigazine.net/')


for card in driver.find_elements_by_css_selector('.card>h2>a>span'):
    print(card.text)