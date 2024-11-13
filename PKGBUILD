# Maintainer: Daniil Redchin <redchindaniil@gmail.com> <github.com/USSURATONCACHI>
pkgname=activate-linux-toggle
pkgver=1.0.1
pkgrel=1
pkgdesc="CLI tool to easily run programs with access to only one network device"
arch=('any')
url="https://github.com/USSURATONCACHI/activate-linux-toggle"
license=('Apache-2.0')
provides=('activate-linux-toggle' 'activate-linux-enable' 'activate-linux-disable' 'activate-linux-is-enabled')

depends=('bash' 'coreutils' 'systemd' 'gtk4')
makedepends=('coreutils' 'rust' 'cargo')

source=("${url}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('7dcd0d4f6b9d4fc6a57cc874d5e5e2c1600d0f6de9459ee17c7f8adeb47cb5a2')

build() {
    novpn_srcdir="${srcdir}/activate-linux-toggle-${pkgver}"

    cd "${novpn_srcdir}" && cargo build --release
}

package() {
    novpn_srcdir="${srcdir}/activate-linux-toggle-${pkgver}"
    
    install -Dm755 "${novpn_srcdir}/target/release/activate-linux-toggle"  "${pkgdir}/usr/bin/activate-linux-toggle"

    install -Dm755 "${novpn_srcdir}/activate-linux-enable"     "${pkgdir}/usr/bin/activate-linux-enable"
    install -Dm755 "${novpn_srcdir}/activate-linux-disable"    "${pkgdir}/usr/bin/activate-linux-disable"
    install -Dm755 "${novpn_srcdir}/activate-linux-is-enabled" "${pkgdir}/usr/bin/activate-linux-is-enabled"

    install -Dm644 "${novpn_srcdir}/activate-linux.service" "${pkgdir}/etc/activate-linux-toggle/activate-linux.service"

    install -Dm644 "${novpn_srcdir}/activate-linux-toggle.desktop" "${pkgdir}/usr/share/applications/activate-linux-toggle.desktop"
}
