use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::{MakeTlsConnector};

fn main() {
    let mut connector = &mut TlsConnector::builder();

    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        connector = connector.add_root_certificate(Certificate::from_der(&cert.0).unwrap())
    }

    let connector = MakeTlsConnector::new(connector.build().unwrap());

    let mut client = postgres::Client::connect(
        "postgres://tsdbadmin:ug2zlznqj3n0su1j@k1dzw28f9f.axbgu744ky.tsdb.cloud.timescale.com:38900/tsdb?sslmode=require",
        connector,
    ).unwrap();

    let x = client.execute("CREATE TABLE measurement(
                    time  TIMESTAMP WITH TIME ZONE NOT NULL,
                    device_id OID NOT NULL,
                    metrics JSONB NOT NULL);",&[]).unwrap();
    println!("{:?}",x);

}





