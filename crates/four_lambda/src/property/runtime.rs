use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Runtime {
    #[serde(rename(serialize = "nodejs"))]
    Nodejs,
    #[serde(rename(serialize = "nodejs4.3"))]
    Nodejs4_3,
    #[serde(rename(serialize = "nodejs6.10"))]
    Nodejs6_10,
    #[serde(rename(serialize = "nodejs8.10"))]
    Nodejs8_10,
    #[serde(rename(serialize = "nodejs10.x"))]
    Nodejs10X,
    #[serde(rename(serialize = "nodejs12.x"))]
    Nodejs12X,
    #[serde(rename(serialize = "nodejs14.x"))]
    Nodejs14X,
    #[serde(rename(serialize = "nodejs16.x"))]
    Nodejs16X,
    #[serde(rename(serialize = "nodejs18.x"))]
    Nodejs18X,
    #[serde(rename(serialize = "nodejs20.x"))]
    Nodejs20X,
    #[serde(rename(serialize = "java8"))]
    Java8,
    #[serde(rename(serialize = "java8.al2"))]
    Java8al2,
    #[serde(rename(serialize = "java11"))]
    Java11,
    #[serde(rename(serialize = "java17"))]
    Java17,
    #[serde(rename(serialize = "java21"))]
    Java21,
    #[serde(rename(serialize = "python2.7"))]
    Python2_7,
    #[serde(rename(serialize = "python3.6"))]
    Python3_6,
    #[serde(rename(serialize = "python3.7"))]
    Python3_7,
    #[serde(rename(serialize = "python3.8"))]
    Python3_8,
    #[serde(rename(serialize = "python3.9"))]
    Python3_9,
    #[serde(rename(serialize = "python3.10"))]
    Python3_10,
    #[serde(rename(serialize = "python3.11"))]
    Python3_11,
    #[serde(rename(serialize = "python3.12"))]
    Python3_12,
    #[serde(rename(serialize = "dotnetcore1.0"))]
    DotNetCore1_0,
    #[serde(rename(serialize = "dotnetcore2.0"))]
    DotNetCore2_0,
    #[serde(rename(serialize = "dotnetcore2.1"))]
    DotNetCore2_1,
    #[serde(rename(serialize = "dotnetcore3.1"))]
    DotNetCore3_1,
    #[serde(rename(serialize = "dotnet6"))]
    DotNetCore6,
    #[serde(rename(serialize = "dotnet8"))]
    DotNetCore8,
    #[serde(rename(serialize = "nodejs4.3-edge"))]
    Nodejs4_3Edge,
    #[serde(rename(serialize = "go1.x"))]
    Go1X,
    #[serde(rename(serialize = "ruby2.5"))]
    Ruby2_5,
    #[serde(rename(serialize = "ruby2.7"))]
    Ruby2_7,
    #[serde(rename(serialize = "ruby3.2"))]
    Ruby3_2,
    #[serde(rename(serialize = "provided"))]
    Provided,
    #[serde(rename(serialize = "provided.al2"))]
    ProvidedAl2,
    #[serde(rename(serialize = "provided.al2023"))]
    ProvidedAl2023,
}
