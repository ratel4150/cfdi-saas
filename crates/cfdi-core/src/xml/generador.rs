use crate::error::CfdiResult;
use crate::models::cfdi::Cfdi;
use crate::models::concepto::ObjetoImpuesto;
use crate::models::carta_porte::TipoUbicacion;

pub fn generar_xml(cfdi: &Cfdi) -> CfdiResult<String> {
    let mut xml = String::with_capacity(4096);

    // ── Declaración XML ───────────────────────────────────
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');

    // ── Nodo raíz cfdi:Comprobante ────────────────────────
    xml.push_str(&format!(
        r#"<cfdi:Comprobante
  xmlns:cfdi="http://www.sat.gob.mx/cfd/4"
  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://www.sat.gob.mx/cfd/4 http://www.sat.gob.mx/sitio_internet/cfd/4/cfdv40.xsd"
  Version="{version}""#,
        version = cfdi.version,
    ));

    // Serie y Folio van DENTRO del nodo raíz antes del cierre >
    if let Some(serie) = &cfdi.serie {
        xml.push_str(&format!(r#"
  Serie="{serie}""#, serie = escapar_xml(serie)));
    }
    if let Some(folio) = &cfdi.folio {
        xml.push_str(&format!(r#"
  Folio="{folio}""#, folio = escapar_xml(folio)));
    }

    xml.push_str(&format!(
        r#"
  Fecha="{fecha}"
  FormaPago="{forma_pago}"
  SubTotal="{sub_total:.2}"
  Moneda="{moneda}"
  Total="{total:.2}"
  TipoDeComprobante="{tipo}"
  Exportacion="{exportacion}"
  MetodoPago="{metodo_pago}"
  LugarExpedicion="{lugar}"
  Sello=""
  NoCertificado=""
  Certificado="">
"#,
        fecha       = cfdi.fecha,
        forma_pago  = forma_pago_str(&cfdi.forma_pago),
        sub_total   = cfdi.sub_total,
        moneda      = moneda_str(&cfdi.moneda),
        total       = cfdi.total,
        tipo        = tipo_str(&cfdi.tipo_comprobante),
        exportacion = cfdi.exportacion,
        metodo_pago = metodo_pago_str(&cfdi.metodo_pago),
        lugar       = cfdi.lugar_expedicion,
    ));

    // ── cfdi:Emisor ───────────────────────────────────────
    xml.push_str(&format!(
        r#"  <cfdi:Emisor
    Rfc="{rfc}"
    Nombre="{nombre}"
    RegimenFiscal="{regimen}"/>
"#,
        rfc     = escapar_xml(&cfdi.emisor.rfc),
        nombre  = escapar_xml(&cfdi.emisor.nombre),
        regimen = cfdi.emisor.regimen_fiscal.codigo(),
    ));

    // ── cfdi:Receptor ─────────────────────────────────────
    xml.push_str(&format!(
        r#"  <cfdi:Receptor
    Rfc="{rfc}"
    Nombre="{nombre}"
    DomicilioFiscalReceptor="{cp}"
    RegimenFiscalReceptor="{regimen}"
    UsoCFDI="{uso}"/>
"#,
        rfc     = escapar_xml(&cfdi.receptor.rfc),
        nombre  = escapar_xml(&cfdi.receptor.nombre),
        cp      = cfdi.receptor.domicilio_fiscal_receptor,
        regimen = cfdi.receptor.regimen_fiscal_receptor,
        uso     = cfdi.receptor.uso_cfdi.codigo(),
    ));

    // ── cfdi:Conceptos ────────────────────────────────────
    xml.push_str("  <cfdi:Conceptos>\n");
    for concepto in &cfdi.conceptos {
        xml.push_str(&format!(
            r#"    <cfdi:Concepto
      ClaveProdServ="{clave}"
      ClaveUnidad="{unidad}"
      Cantidad="{cantidad}"
      Descripcion="{desc}"
      ValorUnitario="{valor:.2}"
      Importe="{importe:.2}"
      ObjetoImp="{obj_imp}">
"#,
            clave    = concepto.clave_prod_serv,
            unidad   = concepto.clave_unidad,
            cantidad = concepto.cantidad,
            desc     = escapar_xml(&concepto.descripcion),
            valor    = concepto.valor_unitario,
            importe  = concepto.importe,
            obj_imp  = objeto_imp_str(&concepto.objeto_imp),
        ));

        // Impuestos del concepto
        if let Some(impuestos) = &concepto.impuestos {
            xml.push_str("      <cfdi:Impuestos>\n");

            if !impuestos.traslados.is_empty() {
                xml.push_str("        <cfdi:Traslados>\n");
                for t in &impuestos.traslados {
                    xml.push_str(&format!(
                        r#"          <cfdi:Traslado
            Base="{base:.2}"
            Impuesto="{imp}"
            TipoFactor="{factor}"
            TasaOCuota="{tasa:.6}"
            Importe="{importe:.2}"/>
"#,
                        base    = t.base,
                        imp     = t.impuesto,
                        factor  = t.tipo_factor,
                        tasa    = t.tasa_o_cuota,
                        importe = t.importe,
                    ));
                }
                xml.push_str("        </cfdi:Traslados>\n");
            }

            if !impuestos.retenciones.is_empty() {
                xml.push_str("        <cfdi:Retenciones>\n");
                for r in &impuestos.retenciones {
                    xml.push_str(&format!(
                        r#"          <cfdi:Retencion
            Base="{base:.2}"
            Impuesto="{imp}"
            TipoFactor="{factor}"
            TasaOCuota="{tasa:.6}"
            Importe="{importe:.2}"/>
"#,
                        base    = r.base,
                        imp     = r.impuesto,
                        factor  = r.tipo_factor,
                        tasa    = r.tasa_o_cuota,
                        importe = r.importe,
                    ));
                }
                xml.push_str("        </cfdi:Retenciones>\n");
            }

            xml.push_str("      </cfdi:Impuestos>\n");
        }

        xml.push_str("    </cfdi:Concepto>\n");
    }
    xml.push_str("  </cfdi:Conceptos>\n");

    // ── cfdi:Impuestos globales ───────────────────────────
    let total_traslados: f64 = cfdi.conceptos.iter()
        .filter_map(|c| c.impuestos.as_ref())
        .flat_map(|i| i.traslados.iter())
        .map(|t| t.importe)
        .sum();

    if total_traslados > 0.0 {
        xml.push_str(&format!(
            r#"  <cfdi:Impuestos TotalImpuestosTrasladados="{total:.2}">
    <cfdi:Traslados>
      <cfdi:Traslado
        Base="{base:.2}"
        Impuesto="002"
        TipoFactor="Tasa"
        TasaOCuota="0.160000"
        Importe="{total:.2}"/>
    </cfdi:Traslados>
  </cfdi:Impuestos>
"#,
            total = total_traslados,
            base  = cfdi.sub_total,
        ));
    }

    // ── Complemento Carta Porte 3.1 ──────────────────────
    if let Some(cp) = &cfdi.carta_porte {
        xml.push_str(&format!(
            r#"  <cfdi:Complemento>
    <cartaporte31:CartaPorte
      xmlns:cartaporte31="http://www.sat.gob.mx/CartaPorte31"
      xsi:schemaLocation="http://www.sat.gob.mx/CartaPorte31 http://www.sat.gob.mx/sitio_internet/cfd/CartaPorte/CartaPorte31.xsd"
      Version="{version}"
      TranspInternac="{trans}"
      TotalDistRec="{dist}">
"#,
            version = cp.version,
            trans   = cp.trans_internac,
            dist    = cp.total_dist_rec,
        ));

        // Ubicaciones
        xml.push_str("      <cartaporte31:Ubicaciones>\n");
        for ub in &cp.ubicaciones {
            let tipo = match ub.tipo_ubicacion {
                TipoUbicacion::Origen  => "Origen",
                TipoUbicacion::Destino => "Destino",
            };
            xml.push_str(&format!(
                r#"        <cartaporte31:Ubicacion
          TipoUbicacion="{tipo}"
          IDUbicacion="{id}"
          RFCRemitenteDestinatario="{rfc}"
          FechaHoraSalidaLlegada="{fecha}""#,
                tipo  = tipo,
                id    = ub.id_ubicacion,
                rfc   = ub.rfc_remitente_dest,
                fecha = ub.fecha_hora_salida_llegada,
            ));
            if let Some(dist) = ub.distancia_recorrida {
                xml.push_str(&format!(r#"
          DistanciaRecorrida="{dist}""#));
            }
            xml.push_str(">\n");
            xml.push_str(&format!(
                r#"          <cartaporte31:Domicilio
            Municipio="{municipio}"
            Estado="{estado}"
            Pais="{pais}"
            CodigoPostal="{cp_dom}"/>
        </cartaporte31:Ubicacion>
"#,
                municipio = ub.domicilio.municipio,
                estado    = ub.domicilio.estado,
                pais      = ub.domicilio.pais,
                cp_dom    = ub.domicilio.codigo_postal,
            ));
        }
        xml.push_str("      </cartaporte31:Ubicaciones>\n");

        // Mercancias
        xml.push_str(&format!(
            r#"      <cartaporte31:Mercancias
        PesoBrutoTotal="{peso}"
        UnidadPeso="{unidad}"
        NumTotalMercancias="{num}">
"#,
            peso   = cp.mercancias.peso_bruto_total,
            unidad = cp.mercancias.unidad_peso,
            num    = cp.mercancias.num_total_mercancias,
        ));
        for m in &cp.mercancias.mercancias {
            xml.push_str(&format!(
                r#"        <cartaporte31:Mercancia
          BienesTransp="{bienes}"
          Descripcion="{desc}"
          Cantidad="{cantidad}"
          ClaveUnidad="{unidad}"
          PesoEnKg="{peso}"
          ValorMercancia="{valor:.2}"
          Moneda="{moneda}"/>
"#,
                bienes   = m.bienes_transp,
                desc     = escapar_xml(&m.descripcion),
                cantidad = m.cantidad,
                unidad   = m.clave_unidad,
                peso     = m.peso_en_kg,
                valor    = m.valor_mercancia,
                moneda   = m.moneda,
            ));
        }

        // Autotransporte
        if let Some(auto) = &cp.mercancias.autotransporte {
            xml.push_str(&format!(
                r#"        <cartaporte31:Autotransporte
          PermSCT="{perm}"
          NumPermisoSCT="{num_perm}"
          ConfigVehicular="{config}"
          PlacaVM="{placa}"
          AnioModeloVM="{anio}">
          <cartaporte31:SeguroRespCivil
            AseguraRespCivil="{asegura}"
            PolizaRespCivil="{poliza}"/>
"#,
                perm     = auto.perm_sct,
                num_perm = auto.num_permiso_sct,
                config   = auto.config_vehicular,
                placa    = auto.placa_vm,
                anio     = auto.anio_modelo_vm,
                asegura  = auto.seguro_resp_civil.asegura_resp_civil,
                poliza   = auto.seguro_resp_civil.poliza_resp_civil,
            ));
            for rem in &auto.remolques {
                xml.push_str(&format!(
                    r#"          <cartaporte31:Remolque SubTipoRem="{sub}" Placa="{placa}"/>
"#,
                    sub   = rem.sub_tipo_rem,
                    placa = rem.placa,
                ));
            }
            xml.push_str("        </cartaporte31:Autotransporte>\n");
        }
        xml.push_str("      </cartaporte31:Mercancias>\n");

        // Figura de transporte
        xml.push_str("      <cartaporte31:FiguraTransporte>\n");
        for fig in &cp.figura_transporte {
            let tipo_fig = match fig.tipo_figura {
                crate::models::carta_porte::TipoFigura::Operador    => "01",
                crate::models::carta_porte::TipoFigura::Propietario => "02",
                crate::models::carta_porte::TipoFigura::Arrendador  => "03",
            };
            xml.push_str(&format!(
                r#"        <cartaporte31:TiposFigura
          TipoFigura="{tipo}"
          RFCFigura="{rfc}"
          NombreFigura="{nombre}""#,
                tipo   = tipo_fig,
                rfc    = fig.rfc_figura,
                nombre = escapar_xml(&fig.nombre_figura),
            ));
            if let Some(lic) = &fig.num_licencia {
                xml.push_str(&format!(r#" NumLicencia="{lic}""#));
            }
            xml.push_str("/>\n");
        }
        xml.push_str("      </cartaporte31:FiguraTransporte>\n");
        xml.push_str("    </cartaporte31:CartaPorte>\n");
        xml.push_str("  </cfdi:Complemento>\n");
    }

    xml.push_str("</cfdi:Comprobante>");
    Ok(xml)
}

// ── Helpers ───────────────────────────────────────────────

fn escapar_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

fn forma_pago_str(f: &crate::models::cfdi::FormaPago) -> &str {
    match f {
        crate::models::cfdi::FormaPago::Efectivo                  => "01",
        crate::models::cfdi::FormaPago::TransferenciaElectronica  => "03",
        crate::models::cfdi::FormaPago::TarjetaCredito            => "04",
        crate::models::cfdi::FormaPago::TarjetaDebito             => "28",
        crate::models::cfdi::FormaPago::PorDefinir                => "99",
    }
}

fn metodo_pago_str(m: &crate::models::cfdi::MetodoPago) -> &str {
    match m {
        crate::models::cfdi::MetodoPago::PagoEnUnaSolaExhibicion    => "PUE",
        crate::models::cfdi::MetodoPago::PagoParcialidadesDiferidos  => "PPD",
    }
}

fn moneda_str(m: &crate::models::cfdi::Moneda) -> &str {
    match m {
        crate::models::cfdi::Moneda::PesoMexicano   => "MXN",
        crate::models::cfdi::Moneda::DolarAmericano => "USD",
    }
}

fn tipo_str(t: &crate::models::cfdi::TipoComprobante) -> &str {
    match t {
        crate::models::cfdi::TipoComprobante::Ingreso  => "I",
        crate::models::cfdi::TipoComprobante::Egreso   => "E",
        crate::models::cfdi::TipoComprobante::Traslado => "T",
        crate::models::cfdi::TipoComprobante::Pago     => "P",
    }
}

fn objeto_imp_str(o: &ObjetoImpuesto) -> &str {
    match o {
        ObjetoImpuesto::NoObjetoImpuesto   => "01",
        ObjetoImpuesto::SiObjetoImpuesto   => "02",
        ObjetoImpuesto::SiObjetoNoObligado => "03",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        cfdi::Cfdi,
        emisor::{Emisor, RegimenFiscal},
        receptor::{Receptor, UsoCfdi},
        concepto::Concepto,
    };

    #[test]
    fn test_genera_xml_cfdi_ingreso() {
        let emisor = Emisor::new(
            "XAXX010101000",
            "Transportes Demo SA de CV",
            RegimenFiscal::GeneralLeyPersonasMorales,
        ).unwrap();

        let receptor = Receptor::new(
            "XAXX010101000",
            "Cliente Demo SA de CV",
            "06600",
            "601",
            UsoCfdi::GastosEnGeneral,
        ).unwrap();

        let concepto = Concepto::nuevo_con_iva(
            "78101803",
            "E48",
            1.0,
            "Servicio de flete Monterrey-CDMX",
            18_400.00,
        );

        let cfdi = Cfdi::nuevo_ingreso(
            Some("F"),
            Some("481"),
            "06600",
            emisor,
            receptor,
            vec![concepto],
        );

        let xml = generar_xml(&cfdi).unwrap();

        // Verifica que Serie y Folio están DENTRO del nodo raíz
        assert!(xml.contains(r#"Version="4.0""#));
        assert!(xml.contains(r#"Serie="F""#));
        assert!(xml.contains(r#"Folio="481""#));
        assert!(xml.contains(r#"TipoDeComprobante="I""#));
        assert!(xml.contains(r#"Impuesto="002""#));
        assert!(xml.contains("cfdi:Conceptos"));
        assert!(xml.contains("18400.00"));

        // Verifica que Serie aparece ANTES del primer tag hijo
        let pos_serie     = xml.find(r#"Serie="F""#).unwrap();
        let pos_emisor    = xml.find("<cfdi:Emisor").unwrap();
        assert!(pos_serie < pos_emisor, "Serie debe estar en el nodo raíz, no fuera");

        println!("\n── XML generado ──\n{}\n", xml);
    }

    #[test]
    fn test_calculo_iva_correcto() {
        let emisor = Emisor::new(
            "XAXX010101000",
            "Test",
            RegimenFiscal::GeneralLeyPersonasMorales,
        ).unwrap();
        let receptor = Receptor::new(
            "XAXX010101000", "Test", "06600", "601", UsoCfdi::GastosEnGeneral,
        ).unwrap();

        let concepto = Concepto::nuevo_con_iva(
            "78101803", "E48", 1.0, "Flete test", 10_000.00,
        );

        assert_eq!(concepto.importe, 10_000.00);
        assert_eq!(
            concepto.impuestos.as_ref().unwrap().traslados[0].importe,
            1_600.00
        );

        let cfdi = Cfdi::nuevo_ingreso(
            None, None, "06600", emisor, receptor, vec![concepto],
        );

        assert_eq!(cfdi.sub_total, 10_000.00);
        assert_eq!(cfdi.total, 11_600.00);
    }
}