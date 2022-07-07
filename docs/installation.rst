
============
Installation
============

pip
---

.. code-block:: console

   $ pip install startinpy


If you want to compile it yourself
----------------------------------

1. install latest `Rust <https://www.rust-lang.org/>`_ 
2. install `maturin <https://github.com/PyO3/maturin>`_ 
3. ``maturin build --release``
4. ``cd ./target/wheels/``
5. ``pip install [name-wheel].whl`` will install it to your local Python



Development (to debug the code)
-------------------------------

1. install latest `Rust <https://www.rust-lang.org/>`_ 
2. install `maturin <https://github.com/PyO3/maturin>`_ 
3. compile the rust code and build the Python bindings (in debug mode, thus slow):

  .. code-block:: console 

    $ maturin develop

4. move to another folder, and this shouldn't return any error:
   
  .. code-block:: console

    $ python
    $ import startinpy