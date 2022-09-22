
============
Installation
============

pip
---

.. code-block:: console

   $ pip install startinpy


If you want to compile it yourself
----------------------------------

1. get the code: https://github.com/hugoledoux/startinpy
2. install latest `Rust <https://www.rust-lang.org/>`_ 
3. install `maturin <https://github.com/PyO3/maturin>`_ 
4. ``maturin build --release``
5. ``cd ./target/wheels/``
6. ``pip install [name-wheel].whl`` will install it to your local Python



Development (to debug the code)
-------------------------------

1. get the code: https://github.com/hugoledoux/startinpy
2. install latest `Rust <https://www.rust-lang.org/>`_ 
3. install `maturin <https://github.com/PyO3/maturin>`_ 
4. compile the rust code and build the Python bindings (in debug mode, thus slow):

  .. code-block:: console 

    $ maturin develop

5. move to another folder, and this shouldn't return any error:
   
  .. code-block:: console

    $ python
    $ import startinpy