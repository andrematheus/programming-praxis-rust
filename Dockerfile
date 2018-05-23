FROM andreroquem/rust-build

MAINTAINER André Roque Matheus <amatheus@mac.com>

VOLUME/app

CMD cd /app; cargo test
