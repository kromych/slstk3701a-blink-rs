#[doc = "Register `CH9_EVAL` reader"]
pub type R = crate::R<Ch9EvalSpec>;
#[doc = "Register `CH9_EVAL` writer"]
pub type W = crate::W<Ch9EvalSpec>;
#[doc = "Field `COMPTHRES` reader - Decision Threshold for Sensor Data"]
pub type CompthresR = crate::FieldReader<u16>;
#[doc = "Field `COMPTHRES` writer - Decision Threshold for Sensor Data"]
pub type CompthresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COMP` reader - Select Mode for Threshold Comparison"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - Select Mode for Threshold Comparison"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECODE` reader - Send Result to Decoder"]
pub type DecodeR = crate::BitReader;
#[doc = "Field `DECODE` writer - Send Result to Decoder"]
pub type DecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Storing of Sensor Sample in Result Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strsample {
    #[doc = "0: Nothing will be stored in the result buffer."]
    Disable = 0,
    #[doc = "1: The sensor sample data will be stored in the result buffer."]
    Data = 1,
    #[doc = "2: The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    Datasrc = 2,
}
impl From<Strsample> for u8 {
    #[inline(always)]
    fn from(variant: Strsample) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strsample {
    type Ux = u8;
}
impl crate::IsEnum for Strsample {}
#[doc = "Field `STRSAMPLE` reader - Enable Storing of Sensor Sample in Result Buffer"]
pub type StrsampleR = crate::FieldReader<Strsample>;
impl StrsampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Strsample> {
        match self.bits {
            0 => Some(Strsample::Disable),
            1 => Some(Strsample::Data),
            2 => Some(Strsample::Datasrc),
            _ => None,
        }
    }
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Strsample::Disable
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Strsample::Data
    }
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn is_datasrc(&self) -> bool {
        *self == Strsample::Datasrc
    }
}
#[doc = "Field `STRSAMPLE` writer - Enable Storing of Sensor Sample in Result Buffer"]
pub type StrsampleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Strsample>;
impl<'a, REG> StrsampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Strsample::Disable)
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Strsample::Data)
    }
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn datasrc(self) -> &'a mut crate::W<REG> {
        self.variant(Strsample::Datasrc)
    }
}
#[doc = "Field `SCANRESINV` reader - Enable Inversion of Result"]
pub type ScanresinvR = crate::BitReader;
#[doc = "Field `SCANRESINV` writer - Enable Inversion of Result"]
pub type ScanresinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configure Evaluation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Threshold comparison is used to evaluate sensor result"]
    Thres = 0,
    #[doc = "1: Sliding window is used to evaluate sensor result"]
    Slidingwin = 1,
    #[doc = "2: Step detection is used to evaluate sensor result"]
    Stepdet = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Configure Evaluation Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Thres),
            1 => Some(Mode::Slidingwin),
            2 => Some(Mode::Stepdet),
            _ => None,
        }
    }
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == Mode::Thres
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn is_slidingwin(&self) -> bool {
        *self == Mode::Slidingwin
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn is_stepdet(&self) -> bool {
        *self == Mode::Stepdet
    }
}
#[doc = "Field `MODE` writer - Configure Evaluation Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn thres(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Thres)
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn slidingwin(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Slidingwin)
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn stepdet(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Stepdet)
    }
}
impl R {
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline(always)]
    pub fn compthres(&self) -> CompthresR {
        CompthresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DecodeR {
        DecodeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline(always)]
    pub fn strsample(&self) -> StrsampleR {
        StrsampleR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline(always)]
    pub fn scanresinv(&self) -> ScanresinvR {
        ScanresinvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline(always)]
    pub fn compthres(&mut self) -> CompthresW<'_, Ch9EvalSpec> {
        CompthresW::new(self, 0)
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, Ch9EvalSpec> {
        CompW::new(self, 16)
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline(always)]
    pub fn decode(&mut self) -> DecodeW<'_, Ch9EvalSpec> {
        DecodeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline(always)]
    pub fn strsample(&mut self) -> StrsampleW<'_, Ch9EvalSpec> {
        StrsampleW::new(self, 18)
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline(always)]
    pub fn scanresinv(&mut self) -> ScanresinvW<'_, Ch9EvalSpec> {
        ScanresinvW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Ch9EvalSpec> {
        ModeW::new(self, 21)
    }
}
#[doc = "Scan Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_eval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_eval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch9EvalSpec;
impl crate::RegisterSpec for Ch9EvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch9_eval::R`](R) reader structure"]
impl crate::Readable for Ch9EvalSpec {}
#[doc = "`write(|w| ..)` method takes [`ch9_eval::W`](W) writer structure"]
impl crate::Writable for Ch9EvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH9_EVAL to value 0"]
impl crate::Resettable for Ch9EvalSpec {}
