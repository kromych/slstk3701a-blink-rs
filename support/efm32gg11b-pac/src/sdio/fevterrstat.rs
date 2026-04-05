#[doc = "Register `FEVTERRSTAT` reader"]
pub type R = crate::R<FevterrstatSpec>;
#[doc = "Register `FEVTERRSTAT` writer"]
pub type W = crate::W<FevterrstatSpec>;
#[doc = "Field `AC12NEX` reader - Force Event for Command Not Issued By Auto CM12 Not Executed"]
pub type Ac12nexR = crate::BitReader;
#[doc = "Field `AC12NEX` writer - Force Event for Command Not Issued By Auto CM12 Not Executed"]
pub type Ac12nexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC12TOE` reader - Force Event for Auto CMD Timeout Error"]
pub type Ac12toeR = crate::BitReader;
#[doc = "Field `AC12TOE` writer - Force Event for Auto CMD Timeout Error"]
pub type Ac12toeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC12CRCE` reader - Force Event for Auto CMD CRC Error"]
pub type Ac12crceR = crate::BitReader;
#[doc = "Field `AC12CRCE` writer - Force Event for Auto CMD CRC Error"]
pub type Ac12crceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC12EBE` reader - Force Event for Auto CMD End Bit Error"]
pub type Ac12ebeR = crate::BitReader;
#[doc = "Field `AC12EBE` writer - Force Event for Auto CMD End Bit Error"]
pub type Ac12ebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC12INDXE` reader - Force Event for Auto CMD Index Error"]
pub type Ac12indxeR = crate::BitReader;
#[doc = "Field `AC12INDXE` writer - Force Event for Auto CMD Index Error"]
pub type Ac12indxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNIBAC12E` reader - Force Event for Command Not Issued By Auto CMD12 Error"]
pub type Cnibac12eR = crate::BitReader;
#[doc = "Field `CNIBAC12E` writer - Force Event for Command Not Issued By Auto CMD12 Error"]
pub type Cnibac12eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTOE` reader - Force Event for Command Timeout Error"]
pub type CmdtoeR = crate::BitReader;
#[doc = "Field `CMDTOE` writer - Force Event for Command Timeout Error"]
pub type CmdtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCRCE` reader - Force Event for Command CRC Error"]
pub type CmdcrceR = crate::BitReader;
#[doc = "Field `CMDCRCE` writer - Force Event for Command CRC Error"]
pub type CmdcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDEBE` reader - Force Event for Command End Bit Error"]
pub type CmdebeR = crate::BitReader;
#[doc = "Field `CMDEBE` writer - Force Event for Command End Bit Error"]
pub type CmdebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDINDXE` reader - Force Event for Command Index Error"]
pub type CmdindxeR = crate::BitReader;
#[doc = "Field `CMDINDXE` writer - Force Event for Command Index Error"]
pub type CmdindxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATTOE` reader - Force Event for Data Timeout Error"]
pub type DattoeR = crate::BitReader;
#[doc = "Field `DATTOE` writer - Force Event for Data Timeout Error"]
pub type DattoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATCRCE` reader - Force Event for Data CRC Error"]
pub type DatcrceR = crate::BitReader;
#[doc = "Field `DATCRCE` writer - Force Event for Data CRC Error"]
pub type DatcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATEBE` reader - Force Event for Data End Bit Error"]
pub type DatebeR = crate::BitReader;
#[doc = "Field `DATEBE` writer - Force Event for Data End Bit Error"]
pub type DatebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURLIMITE` reader - Force Event for Current Limit Error"]
pub type CurlimiteR = crate::BitReader;
#[doc = "Field `CURLIMITE` writer - Force Event for Current Limit Error"]
pub type CurlimiteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC12E` reader - Force Event for Auto CMD Error"]
pub type Ac12eR = crate::BitReader;
#[doc = "Field `AC12E` writer - Force Event for Auto CMD Error"]
pub type Ac12eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMAE` reader - Force Event for ADMA Error"]
pub type AdmaeR = crate::BitReader;
#[doc = "Field `ADMAE` writer - Force Event for ADMA Error"]
pub type AdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNINGE` reader - Force Event for Tuning Errro"]
pub type TuningeR = crate::BitReader;
#[doc = "Field `VENSPECE` reader - Force Event for Vendox Specific Error Status"]
pub type VenspeceR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Force Event for Command Not Issued By Auto CM12 Not Executed"]
    #[inline(always)]
    pub fn ac12nex(&self) -> Ac12nexR {
        Ac12nexR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> Ac12toeR {
        Ac12toeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crce(&self) -> Ac12crceR {
        Ac12crceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> Ac12ebeR {
        Ac12ebeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indxe(&self) -> Ac12indxeR {
        Ac12indxeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> Cnibac12eR {
        Cnibac12eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtoe(&self) -> CmdtoeR {
        CmdtoeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrce(&self) -> CmdcrceR {
        CmdcrceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdebe(&self) -> CmdebeR {
        CmdebeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdindxe(&self) -> CmdindxeR {
        CmdindxeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn dattoe(&self) -> DattoeR {
        DattoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrce(&self) -> DatcrceR {
        DatcrceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datebe(&self) -> DatebeR {
        DatebeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlimite(&self) -> CurlimiteR {
        CurlimiteR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn ac12e(&self) -> Ac12eR {
        Ac12eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn admae(&self) -> AdmaeR {
        AdmaeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Force Event for Tuning Errro"]
    #[inline(always)]
    pub fn tuninge(&self) -> TuningeR {
        TuningeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Force Event for Vendox Specific Error Status"]
    #[inline(always)]
    pub fn venspece(&self) -> VenspeceR {
        VenspeceR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Not Issued By Auto CM12 Not Executed"]
    #[inline(always)]
    pub fn ac12nex(&mut self) -> Ac12nexW<'_, FevterrstatSpec> {
        Ac12nexW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&mut self) -> Ac12toeW<'_, FevterrstatSpec> {
        Ac12toeW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crce(&mut self) -> Ac12crceW<'_, FevterrstatSpec> {
        Ac12crceW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&mut self) -> Ac12ebeW<'_, FevterrstatSpec> {
        Ac12ebeW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indxe(&mut self) -> Ac12indxeW<'_, FevterrstatSpec> {
        Ac12indxeW::new(self, 4)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&mut self) -> Cnibac12eW<'_, FevterrstatSpec> {
        Cnibac12eW::new(self, 7)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtoe(&mut self) -> CmdtoeW<'_, FevterrstatSpec> {
        CmdtoeW::new(self, 16)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrce(&mut self) -> CmdcrceW<'_, FevterrstatSpec> {
        CmdcrceW::new(self, 17)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdebe(&mut self) -> CmdebeW<'_, FevterrstatSpec> {
        CmdebeW::new(self, 18)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdindxe(&mut self) -> CmdindxeW<'_, FevterrstatSpec> {
        CmdindxeW::new(self, 19)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn dattoe(&mut self) -> DattoeW<'_, FevterrstatSpec> {
        DattoeW::new(self, 20)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrce(&mut self) -> DatcrceW<'_, FevterrstatSpec> {
        DatcrceW::new(self, 21)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datebe(&mut self) -> DatebeW<'_, FevterrstatSpec> {
        DatebeW::new(self, 22)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlimite(&mut self) -> CurlimiteW<'_, FevterrstatSpec> {
        CurlimiteW::new(self, 23)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn ac12e(&mut self) -> Ac12eW<'_, FevterrstatSpec> {
        Ac12eW::new(self, 24)
    }
    #[doc = "Bit 25 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn admae(&mut self) -> AdmaeW<'_, FevterrstatSpec> {
        AdmaeW::new(self, 25)
    }
}
#[doc = "Force Event Register for Auto CMD Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fevterrstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fevterrstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FevterrstatSpec;
impl crate::RegisterSpec for FevterrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fevterrstat::R`](R) reader structure"]
impl crate::Readable for FevterrstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fevterrstat::W`](W) writer structure"]
impl crate::Writable for FevterrstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEVTERRSTAT to value 0"]
impl crate::Resettable for FevterrstatSpec {}
