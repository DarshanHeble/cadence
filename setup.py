from setuptools import setup, find_packages

setup(
    name="cadence",
    version="0.1.0",
    description="Schedule git commits to appear as steady daily progress on GitHub",
    py_modules=["cadence", "push_check", "commit_helper", "config_manager"],
    packages=find_packages(),
    install_requires=[
        "textual>=0.80.0",
        "pytz>=2024.1",
    ],
    entry_points={
        "console_scripts": [
            "cadence=cadence:main",
        ],
    },
    python_requires=">=3.9",
)

