import clsx from 'clsx';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import styles from './index.module.css';

function HomepageHeader() {
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          Mod Aggregator
        </Heading>
        <p className="hero__subtitle">
          Десктопное приложение для агрегации модов из различных сайтов с визуальным конструктором парсеров
        </p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/intro">
            Начать изучение
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  return (
    <Layout
      title="Главная"
      description="Документация Mod Aggregator - десктопного приложения для агрегации модов">
      <HomepageHeader />
      <main>
        <div className="container margin-vert--lg">
          <div className="row">
            <div className="col col--4">
              <div className="card">
                <div className="card__header">
                  <Heading as="h3">Архитектура</Heading>
                </div>
                <div className="card__body">
                  <p>Изучите архитектуру проекта, технологический стек и основные компоненты</p>
                  <Link to="/architecture/overview">Подробнее →</Link>
                </div>
              </div>
            </div>
            <div className="col col--4">
              <div className="card">
                <div className="card__header">
                  <Heading as="h3">API</Heading>
                </div>
                <div className="card__body">
                  <p>Документация всех Tauri команд и API для взаимодействия с backend</p>
                  <Link to="/api/tauri-commands">Подробнее →</Link>
                </div>
              </div>
            </div>
            <div className="col col--4">
              <div className="card">
                <div className="card__header">
                  <Heading as="h3">Руководства</Heading>
                </div>
                <div className="card__body">
                  <p>Руководства по разработке, тестированию и развертыванию</p>
                  <Link to="/guides/development">Подробнее →</Link>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </Layout>
  );
}

