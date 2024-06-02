use crate::common::*;



const OUT_FILE_NAME: &str = "./graph_data/test.png";

pub fn draw_test() -> Result<(), anyhow::Error> {

    // 그래프를 저장할 이미지 파일 경로 설정
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1000, 700)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let start_time = match Utc.with_ymd_and_hms(2024, 1, 1, 21, 15, 33) {
        LocalResult::Single(start_time) => start_time,
        _ => return Err(anyhow!("test"))
    };
    
    let end_time = match Utc.with_ymd_and_hms(2024, 1, 1, 21, 20, 33) {
        LocalResult::Single(end_time) => end_time,
        _ => return Err(anyhow!("test"))
    };

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Line Chart Example", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        //.build_cartesian_2d(0..10, 0..50)?;
        //.build_cartesian_2d(0..10, 0..100)?;  
        .build_cartesian_2d(
            start_time..end_time,
            0..100)?;

    chart.configure_mesh()
        //.disable_y_mesh()
        .disable_x_mesh()
        //.disable_x_axis()
        .x_label_formatter(&|dt| dt.format("%H:%M:%S").to_string())
        .x_desc("Time (s)")
        .y_desc("Value (units)")
        .draw()?;
        
    let data1 = vec![
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 16, 33).unwrap(), 30),
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 17, 33).unwrap(), 10),
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 18, 33).unwrap(), 0),
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 19, 33).unwrap(), 40),
    ];


    let data2 = vec![
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 16, 33).unwrap(), 40),
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 17, 33).unwrap(), 50),
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 18, 33).unwrap(), 11),
        (Utc.with_ymd_and_hms(2024, 1, 1, 21, 19, 33).unwrap(), 77),
    ];
        
    // chart.draw_series(LineSeries::new(
    //     data1.iter().map(|(dt, value)| (*dt, *value)), &RED
    // ))?;

    // chart.draw_series(LineSeries::new(
    //     data2.iter().map(|(dt, value)| (*dt, *value)), &BLUE
    // ))?;
    
    // chart.draw_series(LineSeries::new(
    //     (0..10).map(|x| (x, x * x)), &RED,
    // ))?;
    
    // 여러 데이터 세트 정의
    // let data1 = vec![(0, 10), (1, 20), (2, 40), (3, 60), (4, 80)];
    // let data2 = vec![(0, 20), (1, 40), (2, 60), (3, 80), (4, 100)];
    // let data3 = vec![(0, 30), (1, 50), (2, 70), (3, 90), (4, 100)];

    // // 데이터 세트별로 선 그래프 그리기
    chart.draw_series(LineSeries::new(data1, &RED))?.label("Data Series 1").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart.draw_series(LineSeries::new(data2, &BLUE))?.label("Data Series 2").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    // chart.draw_series(LineSeries::new(data3, &GREEN))?;

    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .border_style(&BLACK).draw()?;
        //.background_style(&BLACK.mix(0.8))
        //.border_style(&BLACK);
    
    // 이미지 파일로 그래프 저장
    root_area.present()?;
    println!("Line chart has been saved to 'plotters-doc-data/line_chart.png'.");
    Ok(())
}