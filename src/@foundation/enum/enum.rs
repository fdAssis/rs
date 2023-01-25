#[derive(Debug)]
enum Colour {
    RED,
    BLUE,
    BLACK,
    WHITE,
}

impl Colour {
    fn colour_name(&self) -> &str {
        match &self {
            Colour::RED => "Vermelho",
            Colour::BLUE => "Azul",
            Colour::BLACK => "Preto",
            Colour::WHITE => "Branco",
        }
    }
}

#[derive(Debug)]
enum Country {
    BRA,
    ARG,
    CHI,
}

impl Country {
    fn country_name(&self) -> &str {
        match &self {
            Country::ARG => "Argentina",
            Country::BRA => "Brasil",
            Country::CHI => "Chile",
        }
    }
}

#[derive(Debug)]
enum LicensePlate {
    NationalPlate {
        country: Country,
        city: String,
        uf: String,
        identification: String,
    },
    InternationalPlate {
        country: Country,
        identification: String,
    },
}

#[derive(Debug)]
struct Car {
    year: u32,
    model: String,
    color: Colour,
    license_plate: Option<LicensePlate>,
}

impl Car {
    fn info(&self) -> String {
        format!(
            "Carro: {}, ano: {}, cor: {:?}, indentificação: {:?}",
            &self.model,
            &self.year,
            &self.color.colour_name(),
            match &self.license_plate {
                Some(LicensePlate::NationalPlate {
                    city,
                    country,
                    identification,
                    uf,
                }) => {
                    format!(
                        "cidade: {city}, pais: {}, placa: {identification}, uf: {uf}",
                        country.country_name()
                    )
                }
                Some(LicensePlate::InternationalPlate {
                    country,
                    identification,
                }) => {
                    format!(
                        "Internacional, pais: {}, placa: {identification}",
                        country.country_name()
                    )
                }
                None => format!("Placa não identificada"),
            }
        )
    }

    fn check(&self) -> bool {
        match &self.license_plate {
            Some(LicensePlate::NationalPlate {
                country,
                city,
                uf,
                identification,
            }) => true,
            Some(LicensePlate::InternationalPlate {
                country,
                identification,
            }) => {
                if let Country::BRA = country {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

fn main() {}
