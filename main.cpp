#include <cxx-qt-gen/minefield.cxxqt.h>

#include <QtCore/QUrl>
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

int main(int argc, char** argv) {
    QGuiApplication app(argc, argv);
    QQmlApplicationEngine engine;

    const QUrl rootComponentUrl(QStringLiteral("qrc:/main.qml"));

    QObject::connect(
        &engine, &QQmlApplicationEngine::objectCreated,
        &app, [rootComponentUrl](auto* obj, const auto& url) {
            if (!obj && rootComponentUrl == url) {
                QCoreApplication::exit(-1);
            }
        },
        Qt::QueuedConnection
    );

    qmlRegisterType<Minefield>("Minesweeper", 1, 0, "Minefield");

    engine.load(rootComponentUrl);

    return QCoreApplication::exec();
}