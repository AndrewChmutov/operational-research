import json
import sys

import matplotlib.pyplot as plt
import seaborn as sns

sns.set_theme()

data = json.load(sys.stdin)

for edge in data["edges"]:
    plt.plot(edge["x"], edge["y"], color="crimson")
sns.scatterplot(x=data["cities"]["x"], y=data["cities"]["y"], s=5)

plt.title("Minimum spanning tree of Poland")
plt.savefig(data["target"])
