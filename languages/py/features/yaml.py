import yaml

with open('./config/config.yml') as file:
    obj = yaml.load(file, Loader=yaml.SafeLoader)
    print(obj)
