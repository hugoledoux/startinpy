
# Comparison of startinpy with other Python libraries

For results, see [this page of the documentation](https://startinpy.rtfd.io).


## To replicate

  1. `pip install -r requirements.txt` 
    - make sure you have LAZ for laspy: `pip install 'laspy[lazrs]'`
    - (for macOS `pip intall triangle` is broken, use `pip install triangle2` instead, see <https://pypi.org/project/triangle2/>)
  2. put in same folder `Delaunator.py` from <https://github.com/HakanSeven12/Delaunator-Python> (there is no installer)
  3. download the 2 LAZ files:
    - `wget https://geotiles.citg.tudelft.nl/AHN4_T/04GN2_21.LAZ`
    - `wget https://geotiles.citg.tudelft.nl/AHN4_T/69EZ1_21.LAZ`
  4. `python comparisons.py`, this generates a summary table in Markdown

