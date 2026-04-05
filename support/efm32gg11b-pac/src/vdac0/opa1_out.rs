#[doc = "Register `OPA1_OUT` reader"]
pub type R = crate::R<Opa1OutSpec>;
#[doc = "Register `OPA1_OUT` writer"]
pub type W = crate::W<Opa1OutSpec>;
#[doc = "Field `MAINOUTEN` reader - OPAx Main Output Enable"]
pub type MainoutenR = crate::BitReader;
#[doc = "Field `MAINOUTEN` writer - OPAx Main Output Enable"]
pub type MainoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALTOUTEN` reader - OPAx Alternative Output Enable"]
pub type AltoutenR = crate::BitReader;
#[doc = "Field `ALTOUTEN` writer - OPAx Alternative Output Enable"]
pub type AltoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTOUTEN` reader - OPAx Aport Output Enable"]
pub type AportoutenR = crate::BitReader;
#[doc = "Field `APORTOUTEN` writer - OPAx Aport Output Enable"]
pub type AportoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT` reader - OPAx Main and Alternative Output Short"]
pub type ShortR = crate::BitReader;
#[doc = "Field `SHORT` writer - OPAx Main and Alternative Output Short"]
pub type ShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "OPAx Output Enable Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Altoutpaden {
    #[doc = "1: Alternate Output 0"]
    Out0 = 1,
    #[doc = "2: Alternate Output 1"]
    Out1 = 2,
    #[doc = "4: Alternate Output 2"]
    Out2 = 4,
    #[doc = "8: Alternate Output 3"]
    Out3 = 8,
    #[doc = "16: Alternate Output 4"]
    Out4 = 16,
}
impl From<Altoutpaden> for u8 {
    #[inline(always)]
    fn from(variant: Altoutpaden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Altoutpaden {
    type Ux = u8;
}
impl crate::IsEnum for Altoutpaden {}
#[doc = "Field `ALTOUTPADEN` reader - OPAx Output Enable Value"]
pub type AltoutpadenR = crate::FieldReader<Altoutpaden>;
impl AltoutpadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Altoutpaden> {
        match self.bits {
            1 => Some(Altoutpaden::Out0),
            2 => Some(Altoutpaden::Out1),
            4 => Some(Altoutpaden::Out2),
            8 => Some(Altoutpaden::Out3),
            16 => Some(Altoutpaden::Out4),
            _ => None,
        }
    }
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == Altoutpaden::Out0
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == Altoutpaden::Out1
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn is_out2(&self) -> bool {
        *self == Altoutpaden::Out2
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn is_out3(&self) -> bool {
        *self == Altoutpaden::Out3
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn is_out4(&self) -> bool {
        *self == Altoutpaden::Out4
    }
}
#[doc = "Field `ALTOUTPADEN` writer - OPAx Output Enable Value"]
pub type AltoutpadenW<'a, REG> = crate::FieldWriter<'a, REG, 5, Altoutpaden>;
impl<'a, REG> AltoutpadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut crate::W<REG> {
        self.variant(Altoutpaden::Out0)
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut crate::W<REG> {
        self.variant(Altoutpaden::Out1)
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn out2(self) -> &'a mut crate::W<REG> {
        self.variant(Altoutpaden::Out2)
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn out3(self) -> &'a mut crate::W<REG> {
        self.variant(Altoutpaden::Out3)
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn out4(self) -> &'a mut crate::W<REG> {
        self.variant(Altoutpaden::Out4)
    }
}
#[doc = "Field `APORTOUTSEL` reader - OPAx APORT Output"]
pub type AportoutselR = crate::FieldReader;
#[doc = "Field `APORTOUTSEL` writer - OPAx APORT Output"]
pub type AportoutselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    pub fn mainouten(&self) -> MainoutenR {
        MainoutenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    pub fn altouten(&self) -> AltoutenR {
        AltoutenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> AportoutenR {
        AportoutenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    pub fn short(&self) -> ShortR {
        ShortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    pub fn altoutpaden(&self) -> AltoutpadenR {
        AltoutpadenR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> AportoutselR {
        AportoutselR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    pub fn mainouten(&mut self) -> MainoutenW<'_, Opa1OutSpec> {
        MainoutenW::new(self, 0)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    pub fn altouten(&mut self) -> AltoutenW<'_, Opa1OutSpec> {
        AltoutenW::new(self, 1)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    pub fn aportouten(&mut self) -> AportoutenW<'_, Opa1OutSpec> {
        AportoutenW::new(self, 2)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    pub fn short(&mut self) -> ShortW<'_, Opa1OutSpec> {
        ShortW::new(self, 3)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    pub fn altoutpaden(&mut self) -> AltoutpadenW<'_, Opa1OutSpec> {
        AltoutpadenW::new(self, 4)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    pub fn aportoutsel(&mut self) -> AportoutselW<'_, Opa1OutSpec> {
        AportoutselW::new(self, 16)
    }
}
#[doc = "Operational Amplifier Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa1_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa1_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opa1OutSpec;
impl crate::RegisterSpec for Opa1OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa1_out::R`](R) reader structure"]
impl crate::Readable for Opa1OutSpec {}
#[doc = "`write(|w| ..)` method takes [`opa1_out::W`](W) writer structure"]
impl crate::Writable for Opa1OutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPA1_OUT to value 0x01"]
impl crate::Resettable for Opa1OutSpec {
    const RESET_VALUE: u32 = 0x01;
}
