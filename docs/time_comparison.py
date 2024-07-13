import matplotlib.pyplot as plt

# 定义语言和它们的执行用时
languages = ["Python", "Node.js", "Rust", "Go"]
times = [155.60, 388.56, 99.50, 160.93]  # 对应的执行用时数据

# 数据排序，将时间最长的放在最上面（图表的末尾）
sorted_data = sorted(zip(times, languages))
times, languages = zip(*sorted_data)

# 创建条形图
plt.figure()  # 设置图形的显示大小
plt.barh(languages, times, height=0.5)  # 绘制条形图

# 添加标题和标签
plt.title("Execution Time Comparison in WSL")
plt.xlabel("Programming Languages")
plt.ylabel("Execution Time (s)")

# 显示数值标签
for i, time in enumerate(times):
    plt.text(time, i, f"{time:.2f}", va="center", color="black")

# 显示图形
plt.tight_layout()
plt.show()
