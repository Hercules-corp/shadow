// Middleware for Shadow backend - Request processing and logging
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, web,
};
use actix_web::middleware::Next;
use actix_web::http::header::{HeaderName, HeaderValue};
use std::time::Instant;
use tracing::{info, warn};
use crate::metrics::MetricsCollector;

/// Request timing middleware with metrics collection
pub async fn timing_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let start_time = Instant::now();
    let path = req.path().to_string();
    let method = req.method().to_string();
    
    // Get metrics before calling next (which moves req)
    let metrics_opt = req.app_data::<web::Data<MetricsCollector>>().cloned();
    
    let res = next.call(req).await?;
    
    let duration = start_time.elapsed();
    let status = res.status().as_u16();
    let success = status < 400;
    let duration_ms = duration.as_millis() as u64;
    
    // Record metrics if available
    if let Some(metrics) = metrics_opt {
        metrics.record_request(success, duration_ms, &path);
    }
    
    if status >= 400 {
        warn!(
            "{} {} - {} - {}ms",
            method,
            path,
            status,
            duration_ms
        );
    } else {
        info!(
            "{} {} - {} - {}ms",
            method,
            path,
            status,
            duration_ms
        );
    }
    
    Ok(res)
}

/// Request ID middleware - adds unique ID to each request
pub async fn request_id_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    use uuid::Uuid;
    let request_id = Uuid::new_v4().to_string();
    req.extensions_mut().insert(request_id.clone());
    
    let mut res = next.call(req).await?;
    res.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id).unwrap(),
    );
    
    Ok(res)
}

/// Security headers middleware
pub async fn security_headers_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let mut res = next.call(req).await?;
    
    res.headers_mut().insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    
    res.headers_mut().insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    
    res.headers_mut().insert(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );
    
    Ok(res)
}


